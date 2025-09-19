CC = gcc
CFLAGS = -std=c17 -Wall -Wextra -Wpedantic -Wshadow -Wconversion -Wsign-conversion -Wuninitialized -Werror -fdiagnostics-color=always

SRC = variables.c
OUT = variables

all: $(OUT)

$(OUT): $(SRC)
	$(CC) $(CFLAGS) $(SRC) -o $(OUT)

run: $(OUT)
	./$(OUT)

clean:
	rm -f $(OUT)
