package tree_sitter_calc_test

import (
	"testing"

	tree_sitter "github.com/tree-sitter/go-tree-sitter"
	tree_sitter_calc "github.com/tree-sitter/tree-sitter-calc/bindings/go"
)

func TestCanLoadGrammar(t *testing.T) {
	language := tree_sitter.NewLanguage(tree_sitter_calc.Language())
	if language == nil {
		t.Errorf("Error loading Calc grammar")
	}
}
