
It looks good to me. Now let us look at the next part of the program:

### Part 3

```c
static int
getchr(void)
{
	if (!*gargv)
		return((int)'\0');
	return((int)**gargv++);
}

static char *
getstr(void)
{
	if (!*gargv)
		return("");
	return(*gargv++);
}

static char *number = "+-.0123456789";
static int
getint(void)
{
	if (!*gargv)
		return(0);

	if (strchr(number, **gargv))
		return(atoi(*gargv++));

	return 0;
}

static long
getlong(void)
{
	long val;
	char *ep;

	if (!*gargv)
		return(0L);

	if (**gargv == '\"' || **gargv == '\'')
		return (unsigned char) *((*gargv++)+1);

	errno = 0;
	val = strtol (*gargv, &ep, 0);
	check_conversion(*gargv++, ep);
	return val;
}

static unsigned long
getulong(void)
{
	unsigned long val;
	char *ep;

	if (!*gargv)
		return(0UL);

	if (**gargv == '\"' || **gargv == '\'')
		return (unsigned char) *((*gargv++)+1);

	errno = 0;
	val = strtoul (*gargv, &ep, 0);
	check_conversion(*gargv++, ep);
	return val;
}

static double
getdouble(void)
{
	double val;
	char *ep;

	if (!*gargv)
		return(0.0);

	if (**gargv == '\"' || **gargv == '\'')
		return (unsigned char) *((*gargv++)+1);

	errno = 0;
	val = strtod (*gargv, &ep);
	check_conversion(*gargv++, ep);
	return val;
}

static void
check_conversion(const char *s, const char *ep)
{
	if (*ep) {
		if (ep == s)
			warnx ("%s: expected numeric value", s);
		else
			warnx ("%s: not completely converted", s);
		rval = 1;
	} else if (errno == ERANGE) {
		errno = ERANGE;
		warn("%s", s);
		rval = 1;
	}
}

static void
usage(void)
{
	(void)fprintf(stderr, "usage: printf format [argument ...]\n");
	exit(1);
}
```

Please translate translate Part 3 of the C program (provided above) to **safe** Rust. Please output code directly without explanation. Please do not skip any functions in this part. Each function should be translated completely without placeholders. DO NOT use `unsafe`.
