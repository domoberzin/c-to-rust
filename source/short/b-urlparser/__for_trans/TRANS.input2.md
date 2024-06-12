
It looks good to me. Now let us look at the next part of the program:

### Part 2

```c
url_data_t *
url_parse (char *url) {
  url_data_t *data = malloc(sizeof(url_data_t));
  if (!data) return NULL;

  data->href = url;
  char *tmp;
  char *tmp_url = strdup(url);
  bool is_ssh = false;

  char *protocol = url_get_protocol(tmp_url);
  if (!protocol) return NULL;
  // length of protocol plus ://
  int protocol_len = (int) strlen(protocol) + 3;
  data->protocol = protocol;

  is_ssh = url_is_ssh(protocol);

  char *auth = malloc(sizeof(char));
  int auth_len = 0;
  if ((tmp = strstr(tmp_url, "@"))) {
    auth = get_part(tmp_url, "%[^@]", protocol_len);
    auth_len = strlen(auth);
    if (auth) auth_len++;
  }

  data->auth = auth;

  char *hostname;

  hostname = (is_ssh)
              ? get_part(tmp_url, "%[^:]", protocol_len + auth_len)
              : get_part(tmp_url, "%[^/]", protocol_len + auth_len);

  if (!hostname) return NULL;
  int hostname_len = (int) strlen(hostname);
  char *tmp_hostname = strdup(hostname);
  data->hostname = hostname;

  char *host = malloc(strlen(tmp_hostname) * sizeof(char));
  sscanf(tmp_hostname, "%[^:]", host);
  if (!host) return NULL;
  int host_len = (int) strlen(host);
  data->host = host;

  char *tmp_path;

  tmp_path = (is_ssh)
              ? get_part(tmp_url, ":%s", protocol_len + auth_len + hostname_len)
              : get_part(tmp_url, "/%s", protocol_len + auth_len + hostname_len);

  char *path = malloc(strlen(tmp_path) * sizeof(char));
  if (!path) return NULL;
  char *fmt = (is_ssh)? "%s" : "/%s";
  sprintf(path, fmt, tmp_path);
  data->path = path;
  free(tmp_path);

  char *pathname = malloc(sizeof(char));
  if (!pathname) return NULL;
  strcat(pathname, "");
  tmp_path = strdup(path);
  sscanf(tmp_path, "%[^? | ^#]", pathname);
  int pathname_len = strlen(pathname);
  data->pathname = pathname;

  char *search = malloc(sizeof(search));
  if (!search) return NULL;
  tmp_path = strff(tmp_path, pathname_len);
  strcat(search, "");
  sscanf(tmp_path, "%[^#]", search);
  data->search = search;
  int search_len = strlen(search);
  free(tmp_path);

  char *query = malloc(sizeof(char));
  if (!query) return NULL;
  sscanf(search, "?%s", query);
  data->query = query;

  char *hash = malloc(sizeof(char));
  if (!hash) return NULL;
  tmp_path = strff(path, pathname_len + search_len);
  strcat(hash, "");
  sscanf(tmp_path, "%s", hash);
  data->hash = hash;
  free(tmp_path);

  char *port = malloc(sizeof(char));
  if (!port) return NULL;

  tmp_hostname = strff(hostname, host_len + 1); // +1 for ':' char
  sscanf(tmp_hostname, "%s", port);
  data->port = port;
  free(tmp_hostname);

  return data;
}
```

Please translate translate Part 2 of the C program (provided above) to **safe** Rust. Please output code directly without explanation. Please do not skip any functions in this part. Each function should be translated completely without placeholders. DO NOT use `unsafe`.
