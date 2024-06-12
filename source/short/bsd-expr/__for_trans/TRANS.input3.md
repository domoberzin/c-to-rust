
It looks good to me. Now let us look at the next part of the program:

### Part 3

```c
struct val *
eval6(void)
{
	struct val     *v;

	if (token == OPERAND) {
		nexttoken(0);
		return tokval;
	} else if (token == RP) {
		nexttoken(0);
		v = eval0();
		if (token != LP)
			error();
		nexttoken(0);
		return v;
	} else
		error();
}

/* Parse and evaluate match (regex) expressions */
struct val *
eval5(void)
{
	regex_t		rp;
	regmatch_t	rm[2];
	char		errbuf[256];
	int		eval;
	struct val     *l, *r;
	struct val     *v;

	l = eval6();
	while (token == MATCH) {
		nexttoken(1);
		r = eval6();

		/* coerce to both arguments to strings */
		to_string(l);
		to_string(r);

		/* compile regular expression */
		if ((eval = regcomp(&rp, r->u.s, 0)) != 0) {
			regerror(eval, &rp, errbuf, sizeof(errbuf));
			errx(2, "%s", errbuf);
		}

		/* compare string against pattern --  remember that patterns
		   are anchored to the beginning of the line */
		if (regexec(&rp, l->u.s, 2, rm, 0) == 0 && rm[0].rm_so == 0) {
			if (rm[1].rm_so >= 0) {
				*(l->u.s + rm[1].rm_eo) = '\0';
				v = make_str(l->u.s + rm[1].rm_so);

			} else {
				v = make_int(rm[0].rm_eo - rm[0].rm_so);
			}
		} else {
			if (rp.re_nsub == 0) {
				v = make_int(0);
			} else {
				v = make_str("");
			}
		}

		/* free arguments and pattern buffer */
		free_value(l);
		free_value(r);
		regfree(&rp);

		l = v;
	}

	return l;
}

/* Parse and evaluate multiplication and division expressions */
struct val *
eval4(void)
{
	const char	*errstr;
	struct val	*l, *r;
	enum token	 op;
	volatile int64_t res;

	l = eval5();
	while ((op = token) == MUL || op == DIV || op == MOD) {
		nexttoken(0);
		r = eval5();

		if (!to_integer(l, &errstr))
			errx(2, "number \"%s\" is %s", l->u.s, errstr);
		if (!to_integer(r, &errstr))
			errx(2, "number \"%s\" is %s", r->u.s, errstr);

		if (op == MUL) {
			res = l->u.i * r->u.i;
			if (r->u.i != 0 && l->u.i != res / r->u.i)
				errx(3, "overflow");
			l->u.i = res;
		} else {
			if (r->u.i == 0) {
				errx(2, "division by zero");
			}
			if (op == DIV) {
				if (l->u.i != INT64_MIN || r->u.i != -1)
					l->u.i /= r->u.i;
				else
					errx(3, "overflow");
			} else {
				if (l->u.i != INT64_MIN || r->u.i != -1)
					l->u.i %= r->u.i;
				else
					l->u.i = 0;
			}
		}

		free_value(r);
	}

	return l;
}
```

Please translate translate Part 3 of the C program (provided above) to **safe** Rust. Please output code directly without explanation. Please do not skip any functions in this part. Each function should be translated completely without placeholders. DO NOT use `unsafe`.
