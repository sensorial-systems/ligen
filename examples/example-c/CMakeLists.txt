CMAKE_MINIMUM_REQUIRED(VERSION 3.0)
PROJECT("example-c")

IF(TARGET ${PROJECT_NAME})
RETURN()
ENDIF()

ADD_SUBDIRECTORY(${CMAKE_SOURCE_DIR}/../example/target/ligen/c/example example)

AUX_SOURCE_DIRECTORY(src SOURCES)

ADD_EXECUTABLE(${PROJECT_NAME} ${SOURCES})
TARGET_LINK_LIBRARIES(${PROJECT_NAME} PUBLIC example)

# TODO: How to move this to library CMakeLists? ADD_CUSTOM_COMMAND cannot be used in INTERFACE libraries. See: https://gitlab.kitware.com/cmake/cmake/-/issues/17366
ADD_CUSTOM_COMMAND(TARGET ${PROJECT_NAME} POST_BUILD
    COMMAND ${CMAKE_COMMAND} -E copy_if_different  # which executes "cmake - E copy_if_different..."
        ${CMAKE_SOURCE_DIR}/../example/target/ligen/rust/example/target/debug/ffi_example.dll
        $<TARGET_FILE_DIR:${PROJECT_NAME}>)
