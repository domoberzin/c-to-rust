
It looks good to me. Now let us look at the next part of the program:

### Part 2

```c

/* coerce to vp to an integer */
int
to_integer(struct val *vp, const char **errstr)
{
	int64_t		r;

	if (errstr != NULL)
		*errstr = NULL;

	if (vp->type == integer)
		return 1;

	if (is_integer(vp, &r, errstr)) {
		free(vp->u.s);
		vp->u.i = r;
		vp->type = integer;
		return 1;
	}

	return 0;
}


/* coerce to vp to an string */
void
to_string(struct val *vp)
{
	char	       *tmp;

	if (vp->type == string)
		return;

	if (asprintf(&tmp, "%lld", vp->u.i) == -1)
		err(3, NULL);

	vp->type = string;
	vp->u.s = tmp;
}

int
is_zero_or_null(struct val *vp)
{
	if (vp->type == integer)
		return vp->u.i == 0;
	else
		return *vp->u.s == 0 || (to_integer(vp, NULL) && vp->u.i == 0);
}

void
nexttoken(int pat)
{
	char	       *p;

	if ((p = *av) == NULL) {
		token = EOI;
		return;
	}
	av++;

	
	if (pat == 0 && p[0] != '\0') {
		if (p[1] == '\0') {
			const char     *x = "|&=<>+-*/%:()";
			char	       *i;	/* index */

			if ((i = strchr(x, *p)) != NULL) {
				token = i - x;
				return;
			}
		} else if (p[1] == '=' && p[2] == '\0') {
			switch (*p) {
			case '<':
				token = LE;
				return;
			case '>':
				token = GE;
				return;
			case '!':
				token = NE;
				return;
			}
		}
	}
	tokval = make_str(p);
	token = OPERAND;
	return;
}

void
error(void)
{
	errx(2, "syntax error");
}
```

Please translate translate Part 2 of the C program (provided above) to **safe** Rust. Please output code directly without explanation. Please do not skip any functions in this part. Each function should be translated completely without placeholders. DO NOT use `unsafe`.
