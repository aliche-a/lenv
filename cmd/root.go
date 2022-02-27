package main

import (
	"github.com/spf13/cobra"
)

var (
	rootDesc     = `A command line tool that executes basic Linux commands.`
	rootDescLong = `lenv is a command line tool that can execute basic
Linux shell commands irrelevant of the underlying operating system.

Primarily, this tool is used as a wrapper around the Windows command line,
converting its commands to the Linux equivalent. The aim is to remove the 
need to set up a Linux environment and reduce the amount of setup needed 
to code on a Windows machine.
`
)


type lenv struct {
	baseOS string // Operating system of the underlying machine

	// read a yaml, json, zshrc, bash_profile, etc. file and translate it to the baseOS's equivalent
	// support use of alias, custom scripts/functions, env vars
}

// New root command
func new() *cobra.Command {
	cmd := &cobra.Command{
		Use:     "lenv",
		Aliases: []string{"lv"},
		Short:   rootDesc,
		Long:    rootDescLong,
		RunE: func(cmd *cobra.Command, args []string) error {

			return nil
		},
	}

	return cmd
}
