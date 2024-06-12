
It looks good to me. Now let us look at the next part of the program:

### Part 4

```c
/* Handle splits for /regexp/ and %regexp% patterns. */
void
do_rexp(const char *expr)
{
	regex_t cre;
	intmax_t nwritten;
	long ofs;
	int first;
	char *ecopy, *ep, *p, *pofs, *re;
	FILE *ofp;

	if ((ecopy = strdup(expr)) == NULL)
		err(1, "strdup");

	re = ecopy + 1;
	if ((pofs = strrchr(ecopy, *expr)) == NULL || pofs[-1] == '\\')
		errx(1, "%s: missing trailing %c", expr, *expr);
	*pofs++ = '\0';

	if (*pofs != '\0') {
		errno = 0;
		ofs = strtol(pofs, &ep, 10);
		if (*ep != '\0' || errno != 0)
			errx(1, "%s: bad offset", pofs);
	} else
		ofs = 0;

	if (regcomp(&cre, re, REG_NOSUB) != 0)
		errx(1, "%s: bad regular expression", re);

	if (*expr == '/')
		/* /regexp/: Save results to a file. */
		ofp = newfile();
	else {
		/* %regexp%: Make a temporary file for overflow. */
		if ((ofp = tmpfile()) == NULL)
			err(1, "tmpfile");
	}

	/* Read and output lines until we get a match. */
	first = 1;
	while ((p = get_line()) != NULL) {
		if (fputs(p, ofp) != 0)
			break;
		if (!first && regexec(&cre, p, 0, NULL, 0) == 0)
			break;
		first = 0;
	}

	if (p == NULL) {
		toomuch(NULL, 0);
		errx(1, "%s: no match", re);
	}

	if (ofs <= 0) {
		/*
		 * Negative (or zero) offset: throw back any lines we should
		 * not have read yet.
		  */
		if (p != NULL) {
			toomuch(ofp, -ofs + 1);
			nwritten = (intmax_t)truncofs;
		} else
			nwritten = (intmax_t)ftello(ofp);
	} else {
		/*
		 * Positive offset: copy the requested number of lines
		 * after the match.
		 */
		while (--ofs > 0 && (p = get_line()) != NULL)
			fputs(p, ofp);
		toomuch(NULL, 0);
		nwritten = (intmax_t)ftello(ofp);
		if (fclose(ofp) != 0)
			err(1, "%s", currfile);
	}

	if (!sflag && *expr == '/')
		printf("%jd\n", nwritten);

	regfree(&cre);
	free(ecopy);
}

/* Handle splits based on line number. */
void
do_lineno(const char *expr)
{
	long lastline, tgtline;
	char *ep, *p;
	FILE *ofp;

	errno = 0;
	tgtline = strtol(expr, &ep, 10);
	if (tgtline <= 0 || errno != 0 || *ep != '\0')
		errx(1, "%s: bad line number", expr);
	lastline = tgtline;
	if (lastline <= lineno)
		errx(1, "%s: can't go backwards", expr);

	while (nfiles < maxfiles - 1) {
		ofp = newfile();
		while (lineno + 1 != lastline) {
			if ((p = get_line()) == NULL)
				errx(1, "%ld: out of range", lastline);
			if (fputs(p, ofp) != 0)
				break;
		}
		if (!sflag)
			printf("%jd\n", (intmax_t)ftello(ofp));
		if (fclose(ofp) != 0)
			err(1, "%s", currfile);
		if (reps-- == 0)
			break;
		lastline += tgtline;
	} 
}

```

Please translate translate Part 4 of the C program (provided above) to **safe** Rust. Please output code directly without explanation. Please do not skip any functions in this part. Each function should be translated completely without placeholders. DO NOT use `unsafe`.
