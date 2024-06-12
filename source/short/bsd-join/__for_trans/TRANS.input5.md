
It looks good to me. Now let us look at the next part of the program:

### Part 5

```c
void
outfield(LINE *lp, u_long fieldno, int out_empty)
{
	if (needsep++)
		putwchar(*tabchar);
	if (!ferror(stdout)) {
		if (lp->fieldcnt <= fieldno || out_empty) {
			if (empty != NULL)
				fputs(empty, stdout);
		} else {
			if (*lp->fields[fieldno] == '\0')
				return;
			fputs(lp->fields[fieldno], stdout);
		}
	}
	if (ferror(stdout))
		err(1, "stdout");
}

/*
 * Convert an output list argument "2.1, 1.3, 2.4" into an array of output
 * fields.
 */
void
fieldarg(char *option)
{
	u_long fieldno, filenum;
	char *end, *token;

	while ((token = strsep(&option, ", \t")) != NULL) {
		if (*token == '\0')
			continue;
		if (token[0] == '0')
			filenum = fieldno = 0;
		else if ((token[0] == '1' || token[0] == '2') &&
		    token[1] == '.') {
			filenum = token[0] - '0';
			fieldno = strtol(token + 2, &end, 10);
			if (*end)
				errx(1, "malformed -o option field");
			if (fieldno == 0)
				errx(1, "field numbers are 1 based");
			--fieldno;
		} else
			errx(1, "malformed -o option field");
		if (olistcnt == olistalloc) {
			OLIST *p;
			u_long newsize = olistalloc + 50;
			if ((p = reallocarray(olist, newsize, sizeof(OLIST)))
			    == NULL)
				err(1, NULL);
			olist = p;
			olistalloc = newsize;
		}
		olist[olistcnt].filenum = filenum;
		olist[olistcnt].fieldno = fieldno;
		++olistcnt;
	}
}
```

Please translate translate Part 5 of the C program (provided above) to **safe** Rust. Please output code directly without explanation. Please do not skip any functions in this part. Each function should be translated completely without placeholders. DO NOT use `unsafe`.
