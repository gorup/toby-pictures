FROM debian:10.5-slim
COPY target/release/toby-pictures /
COPY static/ /static
COPY templates/ /templates
EXPOSE 8000
CMD ["/toby-pictures"]
