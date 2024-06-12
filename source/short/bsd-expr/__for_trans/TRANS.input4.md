
It looks good to me. Now let us look at the next part of the program:

### Part 4

```c
/* Parse and evaluate addition and subtraction expressions */
struct val *
eval3(void)
{
	const char	*errstr;
	struct val	*l, *r;
	enum token	 op;
	volatile int64_t res;

	l = eval4();
	while ((op = token) == ADD || op == SUB) {
		nexttoken(0);
		r = eval4();

		if (!to_integer(l, &errstr))
			errx(2, "number \"%s\" is %s", l->u.s, errstr);
		if (!to_integer(r, &errstr))
			errx(2, "number \"%s\" is %s", r->u.s, errstr);

		if (op == ADD) {
			res = l->u.i + r->u.i;
			if ((l->u.i > 0 && r->u.i > 0 && res <= 0) ||
			    (l->u.i < 0 && r->u.i < 0 && res >= 0))
				errx(3, "overflow");
			l->u.i = res;
		} else {
			res = l->u.i - r->u.i;
			if ((l->u.i >= 0 && r->u.i < 0 && res <= 0) ||
			    (l->u.i < 0 && r->u.i > 0 && res >= 0))
				errx(3, "overflow");
			l->u.i = res;
		}

		free_value(r);
	}

	return l;
}

/* Parse and evaluate comparison expressions */
struct val *
eval2(void)
{
	struct val     *l, *r;
	enum token	op;
	int64_t		v = 0, li, ri;

	l = eval3();
	while ((op = token) == EQ || op == NE || op == LT || op == GT ||
	    op == LE || op == GE) {
		nexttoken(0);
		r = eval3();

		if (is_integer(l, &li, NULL) && is_integer(r, &ri, NULL)) {
			switch (op) {
			case GT:
				v = (li >  ri);
				break;
			case GE:
				v = (li >= ri);
				break;
			case LT:
				v = (li <  ri);
				break;
			case LE:
				v = (li <= ri);
				break;
			case EQ:
				v = (li == ri);
				break;
			case NE:
				v = (li != ri);
				break;
			default:
				break;
			}
		} else {
			to_string(l);
			to_string(r);

			switch (op) {
			case GT:
				v = (strcoll(l->u.s, r->u.s) > 0);
				break;
			case GE:
				v = (strcoll(l->u.s, r->u.s) >= 0);
				break;
			case LT:
				v = (strcoll(l->u.s, r->u.s) < 0);
				break;
			case LE:
				v = (strcoll(l->u.s, r->u.s) <= 0);
				break;
			case EQ:
				v = (strcoll(l->u.s, r->u.s) == 0);
				break;
			case NE:
				v = (strcoll(l->u.s, r->u.s) != 0);
				break;
			default:
				break;
			}
		}

		free_value(l);
		free_value(r);
		l = make_int(v);
	}

	return l;
}
```

Please translate translate Part 4 of the C program (provided above) to **safe** Rust. Please output code directly without explanation. Please do not skip any functions in this part. Each function should be translated completely without placeholders. DO NOT use `unsafe`.
