
It looks good to me. Now let us look at the next part of the program:

### Part 4

```c
char *
url_get_path (char *url) {
  int l = 3;
  char *tmp_path;
  char *protocol = url_get_protocol(url);
  char *auth = url_get_auth(url);
  char *hostname = url_get_hostname(url);


  if (!protocol || !hostname)
    return NULL;

  bool is_ssh = url_is_ssh(protocol);

  l += (int) strlen(protocol) + (int) strlen(hostname);

  if (auth) l+= (int) strlen(auth) +1; // @ symbol

  tmp_path = (is_ssh)
              ? get_part(url, ":%s", l)
              : get_part(url, "/%s", l);

  char *fmt = (is_ssh)? "%s" : "/%s";
  char *path = malloc(strlen(tmp_path) * sizeof(char));
  sprintf(path, fmt, tmp_path);

  if (auth) free(auth);
  free(protocol);
  free(hostname);
  free(tmp_path);

  return path;

}

char *
url_get_search (char *url) {
  char *path = url_get_path(url);
  char *pathname = url_get_pathname(url);
  char *search = malloc(sizeof(char));

  if (!path || !search) return NULL;

  char *tmp_path = strff(path, (int)strlen(pathname));
  strcat(search, "");
  sscanf(tmp_path, "%[^#]", search);

  tmp_path = strrwd(tmp_path, (int)strlen(pathname));

  free(path);
  free(pathname);

  return search;
}

char *
url_get_query (char *url) {
  char *search = url_get_search(url);
  char *query = malloc(sizeof(char));
  if (!search) return NULL;
  sscanf(search, "?%s", query);
  free(search);
  return query;
}

char *
url_get_hash (char *url) {
  char *hash = malloc(sizeof(char));
  if (!hash) return NULL;

  char *path = url_get_path(url);
  if (!path) return NULL;

  char *pathname = url_get_pathname(url);
  if (!pathname) return NULL;
  char *search = url_get_search(url);

  int pathname_len = (int) strlen(pathname);
  int search_len = (int) strlen(search);
  char *tmp_path = strff(path, pathname_len + search_len);

  strcat(hash, "");
  sscanf(tmp_path, "%s", hash);
  tmp_path = strrwd(tmp_path, pathname_len + search_len);
  free(tmp_path);
  free(pathname);
  free(path);
  if (search) free(search);

  return hash;
}
```

Please translate translate Part 4 of the C program (provided above) to **safe** Rust. Please output code directly without explanation. Please do not skip any functions in this part. Each function should be translated completely without placeholders. DO NOT use `unsafe`.
