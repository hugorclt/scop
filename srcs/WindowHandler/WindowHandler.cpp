#include "WindowHandler.hpp"

WindowHandler::WindowHandler(int w, int h, std::string name)
:   _width(w), _height(h), _name(name)
{
    initWindow();
}

void    WindowHandler::initWindow() {
    glfwInit();
    glfwWindowHint(GLFW_CLIENT_API, GLFW_NO_API);
    glfwWindowHint(GLFW_RESIZABLE, GLFW_FALSE);

    _window = glfwCreateWindow(_width, _height, _name.c_str(), nullptr, nullptr);

}

WindowHandler::~WindowHandler() {
    glfwDestroyWindow(_window);
    glfwTerminate();
}

bool WindowHandler::shouldClose() {
    return (glfwWindowShouldClose(_window));
}


