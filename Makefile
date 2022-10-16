# **************************************************************************** #
#                                                                              #
#                                                         :::      ::::::::    #
#    Makefile                                           :+:      :+:    :+:    #
#                                                     +:+ +:+         +:+      #
#    By: adelille <adelille@student.42.fr>          +#+  +:+       +#+         #
#                                                 +#+#+#+#+#+   +#+            #
#    Created: 2020/11/30 19:21:49 by adelille          #+#    #+#              #
#    Updated: 2022/10/16 11:22:28 by adelille         ###   ########.fr        #
#                                                                              #
# **************************************************************************** #

NAME =	n-puzzle
CC =	cargo
RM = 	rm -rf

CCFLAGS = --release

# **************************************************************************** #
#	MAKEFILE	#

MAKEFLAGS += --silent

SHELL := bash

# *************************************************************************** #
#	RULES	#

all:		$(NAME)

$(NAME):
	$(CC) build $(CCFLAGS)
	@cp target/*/$(NAME) . || true

run:		$(NAME)
	$(CC) run

check:
	$(CC) check
	
clean:
	$(CC) clean

fclean:		clean
	@$(RM) $(NAME)

re:			fclean all

.PHONY: all clean fclean re run check

# **************************************************************************** #
