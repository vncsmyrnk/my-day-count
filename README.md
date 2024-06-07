# My Day Count

## Development

```bash
docker run --rm -it \
    -v "$(pwd)"/src:/var/app \
    --workdir /var/app \
    --cpus 2 \
    --name my-day-count \
    rust:1.78-slim bash
```
