FROM mariadb:latest

# Set the root password
ENV MYSQL_ROOT_PASSWORD=root

# Create a new database and user
ENV MYSQL_DATABASE=mydb
ENV MYSQL_USER=kylelocal
ENV MYSQL_PASSWORD=kcb

# Grant all privileges to the new user
COPY ./init.sql /docker-entrypoint-initdb.d/
