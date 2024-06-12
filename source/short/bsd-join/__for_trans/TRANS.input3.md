
It looks good to me. Now let us look at the next part of the program:

### Part 3

```c
char *
mbssep(char **stringp, const wchar_t *wcdelim)
{
	char *s, *p;
	size_t ndelim;
	int i;
	/* tabchar is never more than 2 */
	char mbdelim[2][MB_LEN_MAX + 1];
	size_t mblen[2];

	if ((s = *stringp) == NULL)
		return NULL;
	ndelim = wcslen(wcdelim);
	for (i = 0; i < ndelim; i++) {
		/* wcdelim generated via mbtowc */
		mblen[i] = wctomb(mbdelim[i], wcdelim[i]);
	}
	for (p = s; *p != '\0'; p++) {
		for (i = 0; i < ndelim; i++) {
			if (strncmp(p, mbdelim[i], mblen[i]) == 0) {
				*p = '\0';
				*stringp = p + mblen[i];
				return s;
			}
		}
	}
	*stringp = NULL;
	return s;
}

int
cmp(LINE *lp1, u_long fieldno1, LINE *lp2, u_long fieldno2)
{
	if (lp1->fieldcnt <= fieldno1)
		return lp2->fieldcnt <= fieldno2 ? 0 : -1;
	if (lp2->fieldcnt <= fieldno2)
		return 1;
	return strcmp(lp1->fields[fieldno1], lp2->fields[fieldno2]);
}

void
joinlines(INPUT *F1, INPUT *F2)
{
	u_long cnt1, cnt2;

	/*
	 * Output the results of a join comparison.  The output may be from
	 * either file 1 or file 2 (in which case the first argument is the
	 * file from which to output) or from both.
	 */
	if (F2 == NULL) {
		for (cnt1 = 0; cnt1 < F1->setcnt; ++cnt1)
			outoneline(F1, &F1->set[cnt1]);
		return;
	}
	for (cnt1 = 0; cnt1 < F1->setcnt; ++cnt1)
		for (cnt2 = 0; cnt2 < F2->setcnt; ++cnt2)
			outtwoline(F1, &F1->set[cnt1], F2, &F2->set[cnt2]);
}
```

Please translate translate Part 3 of the C program (provided above) to **safe** Rust. Please output code directly without explanation. Please do not skip any functions in this part. Each function should be translated completely without placeholders. DO NOT use `unsafe`.
