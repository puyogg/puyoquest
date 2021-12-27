import * as Axios from 'axios';

export function AxiosResponse(res: Axios.AxiosResponse): void {
  const url =
    res.config.url +
    (res.config.params ? '?' + new URLSearchParams(res.config.params).toString() : '');
  console.log(`${res.config.method?.toUpperCase()} ${url} - ${res.status} ${res.statusText}`);
}
