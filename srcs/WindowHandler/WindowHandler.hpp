#pragma once
#define GLFW_INCLUDE_VULKAN
#include <GLFW/glfw3.h>
#include <string>

class WindowHandler {
    private:
        GLFWwindow  *_window;
        const int   _width;
        const int   _height;
        std::string _name;

    public:
        WindowHandler(int w, int h, std::string name);
        ~WindowHandler();


        void    initWindow();
        bool    shouldClose();



};