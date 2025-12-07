my %var;

grammar EdgeGram {
	rule TOP {
		<stmt>* %% ';'
	}

	token var_name {
		\w+
	}

	token value {
		\d+
	}

	token func_name {
		'print'
	}

	rule stmt {
		| <let_expr>
		| <assignment>
		| <func_call>
	}

	rule let_expr {
		'let' <var_name> {
			%var{$<var_name>} = 0;
		}
	}

	rule assignment {
		<var_name> '=' <value> {
			%var{~$<var_name>} = +$<value>; 
		}
	}

	rule func_call {
		<func_name> '(' <var_name> ')' {
			say %var{$<var_name>}
				if $<func_name> eq 'print';
		}
	}
}

my $code = 'test.ed'.IO.slurp();
my $result = EdgeGram.parse($code);
# say $result 
