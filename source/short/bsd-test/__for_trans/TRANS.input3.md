
It looks good to me. Now let us look at the next part of the program:

### Part 3

```c
static int
primary(enum token n)
{
	int res;

	if (n == EOI)
		syntax(NULL, "argument expected");
	if (n == LPAREN) {
		res = oexpr(t_lex(*++t_wp));
		if (t_lex(*++t_wp) != RPAREN)
			syntax(NULL, "closing paren expected");
		return res;
	}
	/*
	 * We need this, if not binary operations with more than 4
	 * arguments will always fall into unary.
	 */
	if(t_lex_type(t_wp[1]) == BINOP) {
		t_lex(t_wp[1]);
		if (t_wp_op && t_wp_op->op_type == BINOP)
			return binop();
	}

	if (t_wp_op && t_wp_op->op_type == UNOP) {
		/* unary expression */
		if (*++t_wp == NULL)
			syntax(t_wp_op->op_text, "argument expected");
		switch (n) {
		case STREZ:
			return strlen(*t_wp) == 0;
		case STRNZ:
			return strlen(*t_wp) != 0;
		case FILTT:
			return isatty(getn(*t_wp));
		default:
			return filstat(*t_wp, n);
		}
	}

	return strlen(*t_wp) > 0;
}

static const char *
getnstr(const char *s, int *signum, size_t *len)
{
	const char *p, *start;

	/* skip leading whitespaces */
	p = s;
	while (isspace((unsigned char)*p))
		p++;

	/* accept optional sign */
	if (*p == '-') {
		*signum = -1;
		p++;
	} else {
		*signum = 1;
		if (*p == '+')
			p++;
	}

	/* skip leading zeros */
	while (*p == '0' && isdigit((unsigned char)p[1]))
		p++;

	/* turn 0 always positive */
	if (*p == '0')
		*signum = 1;

	start = p;
	while (isdigit((unsigned char)*p))
		p++;
	*len = p - start;

	/* allow trailing whitespaces */
	while (isspace((unsigned char)*p))
		p++;

	/* validate number */
	if (*p != '\0' || *start == '\0')
		errx(2, "%s: invalid", s);

	return start;
}
```

Please translate translate Part 3 of the C program (provided above) to **safe** Rust. Please output code directly without explanation. Please do not skip any functions in this part. Each function should be translated completely without placeholders. DO NOT use `unsafe`.
