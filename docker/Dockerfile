FROM alpine

WORKDIR /app

COPY dist dist
COPY tsquery_rs .

EXPOSE 3000

CMD ["./tsquery_rs"]