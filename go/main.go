package main

import (
	"net"
	"net/http"

	"github.com/labstack/echo/v4"
)

func main() {
	e := echo.New()
	e.GET("/", root)
	e.POST("/users", createUser)

	l, err := net.Listen("tcp", ":8003")
	if err != nil {
		e.Logger.Fatal(err)
	}
	e.Listener = l
	e.Logger.Fatal(e.Start(""))
}

func root(c echo.Context) error {
	return c.String(http.StatusOK, "Hello from go")
}

type User struct {
	Username string `json:"username"`
}

func createUser(c echo.Context) error {
	user := User{}

	if err := c.Bind(user); err != nil {
		return c.String(http.StatusBadRequest, "invalid request")
	}

	var userResp struct {
		Username string `json:"username"`
		Id       int    `json:"id"`
	}
	userResp.Username = user.Username
	userResp.Id = 0

	return c.JSON(http.StatusOK, userResp)
}
