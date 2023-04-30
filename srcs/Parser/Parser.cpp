#include "Parser.hpp"
#include <cstring> 

const std::map<std::string, Parser::ParseFunc> Parser::_lineType = {
	{"o", &Parser::_parseName},
	{"g", &Parser::_parseGroup},
	{"#", &Parser::_parseComment},
	{"v", &Parser::_parseVertex},
	{"vt", &Parser::_parseTexture},
	{"vn", &Parser::_parseNormals},
	{"f", &Parser::_parseFace},
	{"mttlib", &Parser::_parseMaterial},
	{"usemtl", &Parser::_useMaterial},
	{"s", &Parser::_parseShading},
};

Parser::Parser(std::string path) {
	_file.open(path);
	if (_file.good())
		return ;
	throw new FileError();
}

Parser::~Parser(void) {
	_file.close();
}

/* ---------------------------------- utils --------------------------------- */
std::string Parser::_nextLine(void) {
	std::string str;

	std::getline(_file, str);
	return (str);
}

std::string	Parser::_getFirstWord(std::string line) {
	int lastSpaceIndex = line.find(" ");
	if (lastSpaceIndex == -1)
		throw new FileError();
	
	while (line[lastSpaceIndex] == ' ')
		lastSpaceIndex++;
	return line.substr(lastSpaceIndex, line.size());
}

/* ----------------------------- parse function ----------------------------- */
void	Parser::parseObj(Scene &scene) {
	std::string	line = _nextLine();

	while (!line.empty()) {
		_parseLine(line, scene);
		line = _nextLine();
	}
}

void	Parser::_parseLine(std::string line, Scene &scene) {
	for (auto it = _lineType.begin(); it != _lineType.end(); it++) {
		if (line.starts_with(it->first)) {
			(this->*(_lineType.find(it->first)->second))(line, scene);
		}
	}
}

void	Parser::_parseName(std::string line, Scene &scene) {
	scene.setName(_getFirstWord(line));
}

void	Parser::_parseMaterial(std::string line, Scene &scene) {
}

void	Parser::_parseVertex(std::string line, Scene &scene) {
}

void	Parser::_parseTexture(std::string line, Scene &scene) {
}

void	Parser::_parseNormals(std::string line, Scene &scene) {
}

void	Parser::_parseGroup(std::string line, Scene &scene) {
}

void	Parser::_useMaterial(std::string line, Scene &scene) {
}

void	Parser::_parseShading(std::string line, Scene &scene) {
}

void	Parser::_parseFace(std::string line, Scene &scene) {
}

void	Parser::_parseComment(std::string line, Scene &scene) {
}

