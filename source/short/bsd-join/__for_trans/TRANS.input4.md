
It looks good to me. Now let us look at the next part of the program:

### Part 4

```c
void
outoneline(INPUT *F, LINE *lp)
{
	u_long cnt;

	/*
	 * Output a single line from one of the files, according to the
	 * join rules.  This happens when we are writing unmatched single
	 * lines.  Output empty fields in the right places.
	 */
	if (olist)
		for (cnt = 0; cnt < olistcnt; ++cnt) {
			if (olist[cnt].filenum == F->number)
				outfield(lp, olist[cnt].fieldno, 0);
			else if (olist[cnt].filenum == 0)
				outfield(lp, F->joinf, 0);
			else
				outfield(lp, 0, 1);
		}
	else {
		/*
		 * Output the join field, then the remaining fields from F
		 */
		outfield(lp, F->joinf, 0);
		for (cnt = 0; cnt < lp->fieldcnt; ++cnt)
			if (F->joinf != cnt)
				outfield(lp, cnt, 0);
	}

	putchar('\n');
	if (ferror(stdout))
		err(1, "stdout");
	needsep = 0;
}

void
outtwoline(INPUT *F1, LINE *lp1, INPUT *F2, LINE *lp2)
{
	u_long cnt;

	/* Output a pair of lines according to the join list (if any). */
	if (olist) {
		for (cnt = 0; cnt < olistcnt; ++cnt)
			if (olist[cnt].filenum == 0) {
				if (lp1->fieldcnt >= F1->joinf)
					outfield(lp1, F1->joinf, 0);
				else
					outfield(lp2, F2->joinf, 0);
			} else if (olist[cnt].filenum == 1)
				outfield(lp1, olist[cnt].fieldno, 0);
			else /* if (olist[cnt].filenum == 2) */
				outfield(lp2, olist[cnt].fieldno, 0);
	} else {
		/*
		 * Output the join field, then the remaining fields from F1
		 * and F2.
		 */
		outfield(lp1, F1->joinf, 0);
		for (cnt = 0; cnt < lp1->fieldcnt; ++cnt)
			if (F1->joinf != cnt)
				outfield(lp1, cnt, 0);
		for (cnt = 0; cnt < lp2->fieldcnt; ++cnt)
			if (F2->joinf != cnt)
				outfield(lp2, cnt, 0);
	}
	putchar('\n');
	if (ferror(stdout))
		err(1, "stdout");
	needsep = 0;
}
```

Please translate translate Part 4 of the C program (provided above) to **safe** Rust. Please output code directly without explanation. Please do not skip any functions in this part. Each function should be translated completely without placeholders. DO NOT use `unsafe`.
