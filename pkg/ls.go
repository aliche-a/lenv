package list

type Opts struct {
	// "Content" Flags: changes the content displayed
	all       bool   // do not ignore entries starting with .
	author    bool   // with -l, print the author of each file
	dirs      bool   // list directories themselves, not their contents
	recursive bool   // list subdirectories recursively

	// Format flags: changes the way content is displayed
	columns   bool   // list entries by columns
	long      bool   // use a long listing format
	lines     bool   // list entries by lines instead of by columns

	// Sorting flags
	reverse   bool   // reverse order while sorting

	// Flags requiring arguments
	ignore    string // pattern to ignore; do not list implied entries matching shell PATTERN
}

type Opt func(*Opts)

func SetAll() Opt {
	return func(opts *Opts) {
		opts.all = true
	}
}

func SetDirs() Opt {
	return func(opts *Opts) {
		opts.dirs = true
	}
}

func SetRecursive() Opt {
	return func(opts *Opts) {
		opts.recursive = true
	}
}

func SetLong() Opt {
	return func(opts *Opts) {
		opts.long = true
	}
}

func SetIgnore(pattern string) Opt {
	return func(opts *Opts) {
		opts.ignore = pattern
	}
}

// NewOpts initializes an instance of ls with the given options
// using the functional options pattern
func NewOpts(o ...func(*Opts)) *Opts {
	opts := &Opts{}
	for _, f := range o {
		f(opts)
	}
	return opts
}

// LS is the external facing ls command
func LS(opts *Opts) error {

	return nil
}

func ls(opts *Opts) error {

	return nil
}
