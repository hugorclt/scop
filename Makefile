# **************************************************************************** #
#                                                                              #
#                                                         :::      ::::::::    #
#    Makefile                                           :+:      :+:    :+:    #
#                                                     +:+ +:+         +:+      #
#    By: hugorecolet <hugorecolet@student.42.fr>    +#+  +:+       +#+         #
#                                                 +#+#+#+#+#+   +#+            #
#    Created: 2022/09/19 10:51:56 by hrecolet          #+#    #+#              #
#    Updated: 2023/04/20 19:40:11 by hugorecolet      ###   ########.fr        #
#                                                                              #
# **************************************************************************** #

.DEFAULT_GOAL = all

NAME = scop

CXX ?= c++
CXXFLAGS = -Wall -Wextra -Werror -std=c++2b -g3
INCFLAGS = $(addprefix -I./, $(wildcard $(DIR_SRC)/*))

DIR_SRC = srcs
DIR_OBJ = .obj



SRC = $(shell find $(DIR_SRC) -name '*.cpp' -type f)
INC = $(shell find $(DIR_SRC) -name '*.hpp' -type f)
OBJ = $(addprefix $(DIR_OBJ)/, $(notdir $(SRC:.cpp=.o)))

$(DIR_OBJ):
	mkdir -p $(DIR_OBJ)

$(DIR_OBJ)/%.o : $(SRC) $(INC)
	$(CXX) $(CXXFLAGS) -o $@ -c $< $(INCFLAGS)

$(NAME): $(OBJ)
	$(CXX) $(CXXFLAGS) ${OBJ} -o $(NAME)

all: $(DIR_OBJ) $(NAME)

clean:
	rm -f $(OBJ)

fclean: clean
	rm -f $(NAME)
	rm -rf $(DIR_OBJ)

re: fclean
	$(MAKE) all

.PHONY: all clean fclean re