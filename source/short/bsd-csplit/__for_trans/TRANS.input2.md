
It looks good to me. Now let us look at the next part of the program:

### Part 2

```c
static void
usage(void)
{
	extern const char *__progname;

	fprintf(stderr,
	    "usage: %s [-ks] [-f prefix] [-n number] file args ...\n",
	    __progname);
	exit(1);
}

/* ARGSUSED */
void
handlesig(int sig)
{
	const char msg[] = "csplit: caught signal, cleaning up\n";

	write(STDERR_FILENO, msg, sizeof(msg) - 1);
	cleanup();
	_exit(2);
}

/* Create a new output file. */
FILE *
newfile(void)
{
	FILE *fp;

	if ((size_t)snprintf(currfile, sizeof(currfile), "%s%0*ld", prefix,
	    (int)sufflen, nfiles) >= sizeof(currfile)) {
		errno = ENAMETOOLONG;
		err(1, "%s", currfile);
	}
	if ((fp = fopen(currfile, "w+")) == NULL)
		err(1, "%s", currfile);
	nfiles++;

	return (fp);
}

/* Remove partial output, called before exiting. */
void
cleanup(void)
{
	char fnbuf[PATH_MAX];
	long i;

	if (!doclean)
		return;

	/*
	 * NOTE: One cannot portably assume to be able to call snprintf() from
	 * inside a signal handler.  It is, however, safe to do on OpenBSD.
	 */
	for (i = 0; i < nfiles; i++) {
		snprintf(fnbuf, sizeof(fnbuf), "%s%0*ld", prefix,
		    (int)sufflen, i);
		unlink(fnbuf);
	}
}
```

Please translate translate Part 2 of the C program (provided above) to **safe** Rust. Please output code directly without explanation. Please do not skip any functions in this part. Each function should be translated completely without placeholders. DO NOT use `unsafe`.
