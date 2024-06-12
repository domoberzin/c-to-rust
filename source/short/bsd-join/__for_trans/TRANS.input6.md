
It looks good to me. Now let us look at the next part of the program:

### Part 6

```c
void
obsolete(char **argv)
{
	size_t len;
	char **p, *ap, *t;

	while ((ap = *++argv) != NULL) {
		/* Return if "--". */
		if (ap[0] == '-' && ap[1] == '-')
			return;
		/* skip if not an option */
		if (ap[0] != '-')
			continue;
		switch (ap[1]) {
		case 'a':
			/*
			 * The original join allowed "-a", which meant the
			 * same as -a1 plus -a2.  POSIX 1003.2, Draft 11.2
			 * only specifies this as "-a 1" and "a -2", so we
			 * have to use another option flag, one that is
			 * unlikely to ever be used or accidentally entered
			 * on the command line.  (Well, we could reallocate
			 * the argv array, but that hardly seems worthwhile.)
			 */
			if (ap[2] == '\0' && (argv[1] == NULL ||
			    (strcmp(argv[1], "1") != 0 &&
			    strcmp(argv[1], "2") != 0))) {
				ap[1] = '\01';
				warnx("-a option used without an argument; "
				    "reverting to historical behavior");
			}
			break;
		case 'j':
			/*
			 * The original join allowed "-j[12] arg" and "-j arg".
			 * Convert the former to "-[12] arg".  Don't convert
			 * the latter since getopt(3) can handle it.
			 */
			switch(ap[2]) {
			case '1':
			case '2':
				if (ap[3] != '\0')
					goto jbad;
				ap[1] = ap[2];
				ap[2] = '\0';
				break;
			case '\0':
				break;
			default:
jbad:				warnx("unknown option -- %s", ap + 1);
				usage();
			}
			break;
		case 'o':
			/*
			 * The original join allowed "-o arg arg".
			 * Convert to "-o arg -o arg".
			 */
			if (ap[2] != '\0' || argv[1] == NULL)
				break;
			for (p = argv + 2; *p != NULL; ++p) {
				if (p[0][0] == '0' || ((p[0][0] != '1' &&
				    p[0][0] != '2') || p[0][1] != '.'))
					break;
				len = strlen(*p);
				if (len - 2 != strspn(*p + 2, "0123456789"))
					break;
				if ((t = malloc(len + 3)) == NULL)
					err(1, NULL);
				t[0] = '-';
				t[1] = 'o';
				memmove(t + 2, *p, len + 1);
				*p = t;
			}
			argv = p - 1;
			break;
		}
	}
}

void
usage(void)
{
	int len;
	extern const char *__progname;

	len = strlen(__progname) + sizeof("usage: ");
	(void)fprintf(stderr, "usage: %s [-1 field] [-2 field] "
	    "[-a file_number | -v file_number] [-e string]\n"
	    "%*s[-o list] [-t char] file1 file2\n",
	    __progname, len, "");
	exit(1);
}
```

Please translate translate Part 6 of the C program (provided above) to **safe** Rust. Please output code directly without explanation. Please do not skip any functions in this part. Each function should be translated completely without placeholders. DO NOT use `unsafe`.
