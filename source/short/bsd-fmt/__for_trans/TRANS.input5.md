
It looks good to me. Now let us look at the next part of the program:

### Part 5

```c
/* Get a single line from a stream.  Strip control
 * characters and trailing whitespace, and handle backspaces.
 * Return the address of the buffer containing the line.
 * This can cope with arbitrarily long lines, and with lines
 * without terminating \n.
 * If there are no characters left or an error happens, we
 * return NULL.
 */
static char *
get_line(FILE *stream)
{
	int ch;
	int troff = 0;
	static char *buf = NULL;
	static size_t length = 0;
	size_t len = 0;

	if (buf == NULL) {
		length = 100;
		buf = xreallocarray(NULL, length, 1);
	}

	while ((ch = getc(stream)) != '\n' && ch != EOF) {
		if ((len == 0) && (ch == '.' && !format_troff))
			troff = 1;
		if (troff || ch == '\t' || !iscntrl(ch)) {
			if (len >= length - 1) {
				buf = xreallocarray(buf, length, 2);
				length *= 2;
			}
			buf[len++] = ch;
		} else if (ch == '\b') {
			if (len)
				--len;
		}
	}
	while (len > 0 && isspace((unsigned char)buf[len-1]))
		--len;
	buf[len] = '\0';
	return (len > 0 || ch != EOF) ? buf : NULL;
}

/* (Re)allocate some memory, exiting with an error if we can't.
 */
static void *
xreallocarray(void *ptr, size_t nmemb, size_t size)
{
	void *p;

	p  = reallocarray(ptr, nmemb, size);
	if (p == NULL)
		errx(1, "out of memory");
	return p;
}

void
usage(void)
{
	extern const char *__progname;

	fprintf(stderr,
		"usage: %s [-cmnps] [-d chars] [-l number] [-t number]\n"
		"\t[goal [maximum] | -width | -w width] [file ...]\n",
			__progname);
	exit (1);
}
```

Please translate translate Part 5 of the C program (provided above) to **safe** Rust. Please output code directly without explanation. Please do not skip any functions in this part. Each function should be translated completely without placeholders. DO NOT use `unsafe`.
