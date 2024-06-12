
It looks good to me. Now let us look at the next part of the program:

### Part 1

```c
int
main(int argc, char *argv[])
{
	struct sigaction sa;
	long i;
	int ch;
	const char *expr;
	char *ep, *p;
	FILE *ofp;

	kflag = sflag = 0;
	prefix = "xx";
	sufflen = 2;
	while ((ch = getopt(argc, argv, "f:kn:s")) != -1) {
		switch (ch) {
		case 'f':
			prefix = optarg;
			break;
		case 'k':
			kflag = 1;
			break;
		case 'n':
			errno = 0;
			sufflen = strtol(optarg, &ep, 10);
			if (sufflen <= 0 || *ep != '\0' || errno != 0)
				errx(1, "%s: bad suffix length", optarg);
			break;
		case 's':
			sflag = 1;
			break;
		default:
			usage();
		}
	}

	if (sufflen + strlen(prefix) >= PATH_MAX)
		errx(1, "name too long");

	argc -= optind;
	argv += optind;

	if ((infn = *argv++) == NULL)
		usage();
	if (strcmp(infn, "-") == 0) {
		infile = stdin;
		infn = "stdin";
	} else if ((infile = fopen(infn, "r")) == NULL)
		err(1, "%s", infn);

	if (!kflag) {
		doclean = 1;
		atexit(cleanup);
		sa.sa_flags = 0;
		sa.sa_handler = handlesig;
		sigemptyset(&sa.sa_mask);
		sigaddset(&sa.sa_mask, SIGHUP);
		sigaddset(&sa.sa_mask, SIGINT);
		sigaddset(&sa.sa_mask, SIGTERM);
		sigaction(SIGHUP, &sa, NULL);
		sigaction(SIGINT, &sa, NULL);
		sigaction(SIGTERM, &sa, NULL);
	}

	lineno = 0;
	nfiles = 0;
	truncofs = 0;
	overfile = NULL;

	/* Ensure 10^sufflen < LONG_MAX. */
	for (maxfiles = 1, i = 0; i < sufflen; i++) {
		if (maxfiles > LONG_MAX / 10)
			errx(1, "%ld: suffix too long (limit %ld)",
			    sufflen, i);
		maxfiles *= 10;
	}

	/* Create files based on supplied patterns. */
	while (nfiles < maxfiles - 1 && (expr = *argv++) != NULL) {
		/* Look ahead & see if this pattern has any repetitions. */
		if (*argv != NULL && **argv == '{') {
			errno = 0;
			reps = strtol(*argv + 1, &ep, 10);
			if (reps < 0 || *ep != '}' || errno != 0)
				errx(1, "%s: bad repetition count", *argv + 1);
			argv++;
		} else
			reps = 0;

		if (*expr == '/' || *expr == '%') {
			do {
				do_rexp(expr);
			} while (reps-- != 0 && nfiles < maxfiles - 1);
		} else if (isdigit((unsigned char)*expr))
			do_lineno(expr);
		else
			errx(1, "%s: unrecognised pattern", expr);
	}

	/* Copy the rest into a new file. */
	if (!feof(infile)) {
		ofp = newfile();
		while ((p = get_line()) != NULL && fputs(p, ofp) == 0)
			;
		if (!sflag)
			printf("%jd\n", (intmax_t)ftello(ofp));
		if (fclose(ofp) != 0)
			err(1, "%s", currfile);
	}

	toomuch(NULL, 0);
	doclean = 0;

	return (0);
}
```

Please translate translate Part 1 of the C program (provided above) to **safe** Rust. Please output code directly without explanation. Please do not skip any functions in this part. Each function should be translated completely without placeholders. DO NOT use `unsafe`.
