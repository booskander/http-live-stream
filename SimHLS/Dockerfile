FROM alpine:3.18

RUN apk add --no-cache ffmpeg

# Debug step to confirm the context
RUN ls -la

# Copy the binary
COPY simhls /simhls

EXPOSE 8000

CMD ["./simhls"]
