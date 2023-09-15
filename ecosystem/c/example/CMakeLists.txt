CMAKE_MINIMUM_REQUIRED(VERSION 3.0)
PROJECT("example-c")

IF(TARGET ${PROJECT_NAME})
RETURN()
ENDIF()

ADD_SUBDIRECTORY(deps/ctest)
ADD_SUBDIRECTORY(${CMAKE_SOURCE_DIR}/../../target/ligen/c/example example)
ADD_SUBDIRECTORY(${CMAKE_SOURCE_DIR}/../../target/ligen/c/ligen_rust_runtime ligen_rust_runtime)

AUX_SOURCE_DIRECTORY(src SOURCES)

ADD_EXECUTABLE(${PROJECT_NAME} ${SOURCES})
TARGET_LINK_LIBRARIES(${PROJECT_NAME} PUBLIC ctest example ligen_rust_runtime)
