#include "Scene.hpp"
#include "Parser.hpp"
#include "WindowHandler.hpp"

int main(int ac, char **av) {
    if (ac != 2)
		return (1);

	// Scene	scene;
	// Parser	parser(av[1]);

	// parser.parseObj(scene);
	try {
		WindowHandler	windowHandler(800, 600, "SCOP");

		while (!windowHandler.shouldClose()) {
			glfwPollEvents();
		}


	} catch (std::exception &e) {
		std::cout << e.what() << std::endl;
		return EXIT_FAILURE;
	}
	return EXIT_SUCCESS;
}