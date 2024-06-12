
It looks good to me. Now let us look at the next part of the program:

### Part 5

```c
/* Parse and evaluate & expressions */
struct val *
eval1(void)
{
	struct val     *l, *r;

	l = eval2();
	while (token == AND) {
		nexttoken(0);
		r = eval2();

		if (is_zero_or_null(l) || is_zero_or_null(r)) {
			free_value(l);
			free_value(r);
			l = make_int(0);
		} else {
			free_value(r);
		}
	}

	return l;
}

/* Parse and evaluate | expressions */
struct val *
eval0(void)
{
	struct val     *l, *r;

	l = eval1();
	while (token == OR) {
		nexttoken(0);
		r = eval1();

		if (is_zero_or_null(l)) {
			free_value(l);
			l = r;
		} else {
			free_value(r);
		}
	}

	return l;
}


int
main(int argc, char *argv[])
{
	struct val     *vp;

	if (argc > 1 && !strcmp(argv[1], "--"))
		argv++;

	av = argv + 1;

	nexttoken(0);
	vp = eval0();

	if (token != EOI)
		error();

	if (vp->type == integer)
		printf("%lld\n", vp->u.i);
	else
		printf("%s\n", vp->u.s);

	return is_zero_or_null(vp);
}
```

Please translate translate Part 5 of the C program (provided above) to **safe** Rust. Please output code directly without explanation. Please do not skip any functions in this part. Each function should be translated completely without placeholders. DO NOT use `unsafe`.
