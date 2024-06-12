
It looks good to me. Now let us look at the next part of the program:

### Part 1

```c
// non C99 standard functions
#if __STDC_VERSION__ >= 199901L
char *
strdup (const char *str) {
  int n = strlen(str) + 1;
  char *dup = malloc(n);
  if (dup) strcpy(dup, str);
  return dup;
}
#endif


static char *
strff (char *ptr, int n) {
  int y = 0;
  for (int i = 0; i < n; ++i) {
    y = *ptr++;
  }

  return strdup(ptr);
}

static char *
strrwd (char *ptr, int n) {
  int y = 0;
  for (int i = 0; i < n; ++i) {
    y = *ptr--;
  }

  return strdup(ptr);
}

static char *
get_part (char *url, const char *format, int l) {
  bool has = false;
  char *tmp = malloc(sizeof(char));
  char *tmp_url = strdup(url);
  char *fmt_url = strdup(url);
  char *ret = malloc(sizeof(char));

  if (!tmp || !tmp_url || !fmt_url || !ret)
    return NULL;

  strcpy(tmp, "");
  strcpy(fmt_url, "");

  // move pointer exactly the amount
  // of characters in the `prototcol` char
  // plus 3 characters that represent the `://`
  // part of the url
  fmt_url = strff(fmt_url, l);

  sscanf(fmt_url, format, tmp);

  if (0 != strcmp(tmp, tmp_url)) {
    has = true;
    ret = strdup(tmp);
  }

  // descrement pointer to original
  // position so it can be free'd
  fmt_url = strrwd(fmt_url, l);

  free(tmp);
  free(tmp_url);
  free(fmt_url);

  return has? ret : NULL;
}
```

Please translate translate Part 1 of the C program (provided above) to **safe** Rust. Please output code directly without explanation. Please do not skip any functions in this part. Each function should be translated completely without placeholders. DO NOT use `unsafe`.
