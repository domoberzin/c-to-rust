
It looks good to me. Now let us look at the next part of the program:

### Part 6

```c
static enum token
t_lex(char *s)
{
	struct t_op const *op = ops;

	if (s == 0) {
		t_wp_op = NULL;
		return EOI;
	}
	while (op->op_text) {
		if (strcmp(s, op->op_text) == 0) {
			t_wp_op = op;
			return op->op_num;
		}
		op++;
	}
	t_wp_op = NULL;
	return OPERAND;
}

/* atoi with error detection */
static int
getn(const char *s)
{
	char buf[32];
	const char *errstr, *p;
	size_t len;
	int r, sig;

	p = getnstr(s, &sig, &len);
	if (sig != 1)
		errstr = "too small";
	else if (len >= sizeof(buf))
		errstr = "too large";
	else {
		strlcpy(buf, p, sizeof(buf));
		buf[len] = '\0';
		r = strtonum(buf, 0, INT_MAX, &errstr);
	}

	if (errstr != NULL)
		errx(2, "%s: %s", s, errstr);

	return r;
}

static int
newerf(const char *f1, const char *f2)
{
	struct stat b1, b2;

	return (stat(f1, &b1) == 0 &&
	    stat(f2, &b2) == 0 &&
	    b1.st_mtime > b2.st_mtime);
}

static int
olderf(const char *f1, const char *f2)
{
	struct stat b1, b2;

	return (stat(f1, &b1) == 0 &&
	    stat(f2, &b2) == 0 &&
	    b1.st_mtime < b2.st_mtime);
}

static int
equalf(const char *f1, const char *f2)
{
	struct stat b1, b2;

	return (stat(f1, &b1) == 0 &&
	    stat(f2, &b2) == 0 &&
	    b1.st_dev == b2.st_dev &&
	    b1.st_ino == b2.st_ino);
}
```

Please translate translate Part 6 of the C program (provided above) to **safe** Rust. Please output code directly without explanation. Please do not skip any functions in this part. Each function should be translated completely without placeholders. DO NOT use `unsafe`.
