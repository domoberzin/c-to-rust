
It looks good to me. Now let us look at the next part of the program:

### Part 4

```c
static int
intcmp(const char *opnd1, const char *opnd2)
{
	const char *p1, *p2;
	size_t len1, len2;
	int c, sig1, sig2;

	p1 = getnstr(opnd1, &sig1, &len1);
	p2 = getnstr(opnd2, &sig2, &len2);

	if (sig1 != sig2)
		c = sig1;
	else if (len1 != len2)
		c = (len1 < len2) ? -sig1 : sig1;
	else
		c = strncmp(p1, p2, len1) * sig1;

	return c;
}

static int
binop(void)
{
	const char *opnd1, *opnd2;
	struct t_op const *op;

	opnd1 = *t_wp;
	(void) t_lex(*++t_wp);
	op = t_wp_op;

	if ((opnd2 = *++t_wp) == NULL)
		syntax(op->op_text, "argument expected");

	switch (op->op_num) {
	case STREQ:
		return strcmp(opnd1, opnd2) == 0;
	case STRNE:
		return strcmp(opnd1, opnd2) != 0;
	case STRLT:
		return strcmp(opnd1, opnd2) < 0;
	case STRGT:
		return strcmp(opnd1, opnd2) > 0;
	case INTEQ:
		return intcmp(opnd1, opnd2) == 0;
	case INTNE:
		return intcmp(opnd1, opnd2) != 0;
	case INTGE:
		return intcmp(opnd1, opnd2) >= 0;
	case INTGT:
		return intcmp(opnd1, opnd2) > 0;
	case INTLE:
		return intcmp(opnd1, opnd2) <= 0;
	case INTLT:
		return intcmp(opnd1, opnd2) < 0;
	case FILNT:
		return newerf(opnd1, opnd2);
	case FILOT:
		return olderf(opnd1, opnd2);
	case FILEQ:
		return equalf(opnd1, opnd2);
	}

	syntax(op->op_text, "not a binary operator");
}
```

Please translate translate Part 4 of the C program (provided above) to **safe** Rust. Please output code directly without explanation. Please do not skip any functions in this part. Each function should be translated completely without placeholders. DO NOT use `unsafe`.
