package main

import (
	"github.com/spf13/cobra"
)

var (
	lsDesc     = `List directory contents.`
	lsLongDesc = `List information about the FILEs (the current directory by default).
Sort entries alphabetically if none of -cftuvSUX nor --sort is specified.`
)

func newListCmd() *cobra.Command {
	cmd := &cobra.Command{
		Use:   "ls",
		Short: lsDesc,
		Long:  lsLongDesc,
		Args:  cobra.MinimumNArgs(1),
		RunE: func(cmd *cobra.Command, args []string) error {
			// If a file/directory doesn't exist, just skip it

			return nil
		},
	}

	return cmd
}
