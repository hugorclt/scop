#include "Scene.hpp"
#include "Parser.hpp"

int main(int ac, char **av) {
    if (ac != 2)
		return (1);

	Scene	scene;
	Parser	parser(av[1]);

	parser.parseObj(scene);
}