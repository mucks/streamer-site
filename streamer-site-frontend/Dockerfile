FROM node

WORKDIR /app

COPY package*.json ./
RUN npm install

COPY . .
RUN node_modules/@angular/cli/bin/ng build --prod

VOLUME ["/volume"]
ENTRYPOINT [ "./entrypoint.sh" ]