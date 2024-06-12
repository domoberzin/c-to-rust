
It looks good to me. Now let us look at the next part of the program:

### Part 1

```c
// int  cmp(LINE *, u_long, LINE *, u_long);
// void fieldarg(char *);
// void joinlines(INPUT *, INPUT *);
// char *mbssep(char **, const wchar_t *);
// void obsolete(char **);
// void outfield(LINE *, u_long, int);
// void outoneline(INPUT *, LINE *);
// void outtwoline(INPUT *, LINE *, INPUT *, LINE *);
// void slurp(INPUT *);
// void usage(void);

int
main(int argc, char *argv[])
{
	INPUT *F1, *F2;
	int aflag, ch, cval, vflag;
	char *end;

	setlocale(LC_CTYPE, "");

	F1 = &input1;
	F2 = &input2;

	aflag = vflag = 0;
	obsolete(argv);
	while ((ch = getopt(argc, argv, "\01a:e:j:1:2:o:t:v:")) != -1) {
		switch (ch) {
		case '\01':		/* See comment in obsolete(). */
			aflag = 1;
			F1->unpair = F2->unpair = 1;
			break;
		case '1':
			if ((F1->joinf = strtol(optarg, &end, 10)) < 1)
				errx(1, "-1 option field number less than 1");
			if (*end)
				errx(1, "illegal field number -- %s", optarg);
			--F1->joinf;
			break;
		case '2':
			if ((F2->joinf = strtol(optarg, &end, 10)) < 1)
				errx(1, "-2 option field number less than 1");
			if (*end)
				errx(1, "illegal field number -- %s", optarg);
			--F2->joinf;
			break;
		case 'a':
			aflag = 1;
			switch(strtol(optarg, &end, 10)) {
			case 1:
				F1->unpair = 1;
				break;
			case 2:
				F2->unpair = 1;
				break;
			default:
				errx(1, "-a option file number not 1 or 2");
				break;
			}
			if (*end)
				errx(1, "illegal file number -- %s", optarg);
			break;
		case 'e':
			empty = optarg;
			break;
		case 'j':
			if ((F1->joinf = F2->joinf = strtol(optarg, &end, 10)) < 1)
				errx(1, "-j option field number less than 1");
			if (*end)
				errx(1, "illegal field number -- %s", optarg);
			--F1->joinf;
			--F2->joinf;
			break;
		case 'o':
			fieldarg(optarg);
			break;
		case 't':
			spans = 0;
			if (mbtowc(tabchar, optarg, MB_CUR_MAX) !=
			    strlen(optarg))
				errx(1, "illegal tab character specification");
			tabchar[1] = L'\0';
			break;
		case 'v':
			vflag = 1;
			joinout = 0;
			switch (strtol(optarg, &end, 10)) {
			case 1:
				F1->unpair = 1;
				break;
			case 2:
				F2->unpair = 1;
				break;
			default:
				errx(1, "-v option file number not 1 or 2");
				break;
			}
			if (*end)
				errx(1, "illegal file number -- %s", optarg);
			break;
		case '?':
		default:
			usage();
		}
	}
	argc -= optind;
	argv += optind;

	if (aflag && vflag)
		errx(1, "the -a and -v options are mutually exclusive");

	if (argc != 2)
		usage();

	/* Open the files; "-" means stdin. */
	if (!strcmp(*argv, "-"))
		F1->fp = stdin;
	else if ((F1->fp = fopen(*argv, "r")) == NULL)
		err(1, "%s", *argv);
	++argv;
	if (!strcmp(*argv, "-"))
		F2->fp = stdin;
	else if ((F2->fp = fopen(*argv, "r")) == NULL)
		err(1, "%s", *argv);
	if (F1->fp == stdin && F2->fp == stdin)
		errx(1, "only one input file may be stdin");

	slurp(F1);
	slurp(F2);

	/*
	 * We try to let the files have the same field value, advancing
	 * whoever falls behind and always advancing the file(s) we output
	 * from.
	*/
	while (F1->setcnt && F2->setcnt) {
		cval = cmp(F1->set, F1->joinf, F2->set, F2->joinf);
		if (cval == 0) {
			/* Oh joy, oh rapture, oh beauty divine! */
			if (joinout)
				joinlines(F1, F2);
			slurp(F1);
			slurp(F2);
		} else if (cval < 0) {
			/* File 1 takes the lead... */
			if (F1->unpair)
				joinlines(F1, NULL);
			slurp(F1);
		} else {
			/* File 2 takes the lead... */
			if (F2->unpair)
				joinlines(F2, NULL);
			slurp(F2);
		}
	}

	/*
	 * Now that one of the files is used up, optionally output any
	 * remaining lines from the other file.
	 */
	if (F1->unpair)
		while (F1->setcnt) {
			joinlines(F1, NULL);
			slurp(F1);
		}
	if (F2->unpair)
		while (F2->setcnt) {
			joinlines(F2, NULL);
			slurp(F2);
		}

	return 0;
}
```

Please translate translate Part 1 of the C program (provided above) to **safe** Rust. Please output code directly without explanation. Please do not skip any functions in this part. Each function should be translated completely without placeholders. DO NOT use `unsafe`.
