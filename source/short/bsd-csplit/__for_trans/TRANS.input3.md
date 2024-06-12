
It looks good to me. Now let us look at the next part of the program:

### Part 3

```c
/* Read a line from the input into a static buffer. */
char *
get_line(void)
{
	static char lbuf[LINE_MAX];
	FILE *src;

	src = overfile != NULL ? overfile : infile;

again: if (fgets(lbuf, sizeof(lbuf), src) == NULL) {
		if (src == overfile) {
			src = infile;
			goto again;
		}
		return (NULL);
	}
	if (ferror(src))
		err(1, "%s", infn);
	lineno++;

	return (lbuf);
}

/* Conceptually rewind the input (as obtained by get_line()) back `n' lines. */
void
toomuch(FILE *ofp, long n)
{
	char buf[BUFSIZ];
	size_t i, nread;

	if (overfile != NULL) {
		/*
		 * Truncate the previous file we overflowed into back to
		 * the correct length, close it.
		 */
		if (fflush(overfile) != 0)
			err(1, "overflow");
		if (ftruncate(fileno(overfile), truncofs) != 0)
			err(1, "overflow");
		if (fclose(overfile) != 0)
			err(1, "overflow");
		overfile = NULL;
	}

	if (n == 0)
		/* Just tidying up */
		return;

	lineno -= n;

	/*
	 * Wind the overflow file backwards to `n' lines before the
	 * current one.
	 */
	do {
		if (ftello(ofp) < (off_t)sizeof(buf))
			rewind(ofp);
		else
			fseeko(ofp, -(off_t)sizeof(buf), SEEK_CUR);
		if (ferror(ofp))
			errx(1, "%s: can't seek", currfile);
		if ((nread = fread(buf, 1, sizeof(buf), ofp)) == 0)
			errx(1, "can't read overflowed output");
		if (fseeko(ofp, -(off_t)nread, SEEK_CUR) != 0)
			err(1, "%s", currfile);
		for (i = 1; i <= nread; i++)
			if (buf[nread - i] == '\n' && n-- == 0)
				break;
		if (ftello(ofp) == 0)
			break;
	} while (n > 0);
	if (fseeko(ofp, (off_t)(nread - i + 1), SEEK_CUR) != 0)
		err(1, "%s", currfile);

	/*
	 * get_line() will read from here. Next call will truncate to
	 * truncofs in this file.
	 */
	overfile = ofp;
	truncofs = ftello(overfile);
}
```

Please translate translate Part 3 of the C program (provided above) to **safe** Rust. Please output code directly without explanation. Please do not skip any functions in this part. Each function should be translated completely without placeholders. DO NOT use `unsafe`.
