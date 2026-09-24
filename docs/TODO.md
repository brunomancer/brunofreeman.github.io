# TODO

## Point the apex domain at GitHub Pages

`dev.quest/<slug>` short links don't work yet. The bare domain `dev.quest` resolves to Squarespace domain forwarding (A record `198.49.23.145`). That forwarder sends every path to `https://www.dev.quest` with a 302, so `dev.quest/improv` ends up on the homepage instead of `/improv/`. `www.dev.quest/<slug>` already works.

In Squarespace → Domains → dev.quest → DNS:

- [ ] Remove the domain forwarding rule for `dev.quest` and the `198.49.23.145` A record.
- [ ] Add A records for host `@`: `185.199.108.153`, `185.199.109.153`, `185.199.110.153`, `185.199.111.153`.
- [ ] Optionally add AAAA records for host `@`: `2606:50c0:8000::153`, `2606:50c0:8001::153`, `2606:50c0:8002::153`, `2606:50c0:8003::153`.
- [ ] Wait for GitHub to issue an HTTPS certificate for the bare domain (a few minutes up to about an hour).
- [ ] Verify the redirect chain keeps the path. Expect a 301 to `https://www.dev.quest/improv`, then to `/improv/`:

  ```sh
  curl -sIL https://dev.quest/improv | grep -iE '^(HTTP|location)'
  ```
