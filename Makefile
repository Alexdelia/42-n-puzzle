# **************************************************************************** #
#                                                                              #
#                                                         :::      ::::::::    #
#    Makefile                                           :+:      :+:    :+:    #
#                                                     +:+ +:+         +:+      #
#    By: adelille <adelille@student.42.fr>          +#+  +:+       +#+         #
#                                                 +#+#+#+#+#+   +#+            #
#    Created: 2020/11/30 19:21:49 by adelille          #+#    #+#              #
#    Updated: 2022/10/11 21:02:34 by adelille         ###   ########.fr        #
#                                                                              #
# **************************************************************************** #

NAME =	n-puzzle
CC =	cargo
RM = 	rm -rf

#CCFLAGS =

# **************************************************************************** #
#	MAKEFILE	#

MAKEFLAGS += --silent

SHELL := bash

# *************************************************************************** #
#	RULES	#

all:		$(NAME)

$(NAME):
	$(CC) build
	@cp target/debug/$(NAME) .

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
