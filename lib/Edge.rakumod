unit module Edge;

my %var;

class EdgeActions is export {
    method let_expr($/) {
        %var{~$<var_name>} = $<value>.made // 0;
    }
    
    method assignment($/) {
        %var{~$<var_name>} = $<value>.made;
    }
    
    method func_call($/) {
        say %var{$<var_name>} if $<func_name> eq 'print';
    }
    
    method value($/) {
        make $<expr>.made;
    }
    
    method expr($/) {
        my $result = $<term>[0].made;
        for ^$<addop>.elems -> $i {
            my $op = ~$<addop>[$i];
            my $term = $<term>[$i + 1].made;
            $result = $op eq '+' ?? $result + $term !! $result - $term;
        }
        make $result;
    }
    
    method term($/) {
        my $result = $<factor>[0].made;
        for ^$<mulop>.elems -> $i {
            my $op = ~$<mulop>[$i];
            my $factor = $<factor>[$i + 1].made;
            $result = $op eq '*' ?? $result * $factor !! $result / $factor;
        }
        make $result;
    }
    
    method factor($/) {
        if $<number> {
            make +$<number>;
        } elsif $<var_name> {
            make %var{~$<var_name>} // 0;
        } else {
            make $<expr>.made;
        }
    }
}

grammar EdgeGram is export {
    rule TOP {
        \s* [ <stmt> \s* [ ';' \s* ]? ]*
    }
    
    token var_name { \w+ }
    token number { \d+ }
    
    rule value { <expr> }
    
    rule expr {
        <term> [ <addop> <term> ]*
    }
    
    token addop { '+' | '-' }
    
    rule term {
        <factor> [ <mulop> <factor> ]*
    }
    
    token mulop { '*' | '/' }
    
    rule factor {
        | <number>
        | <var_name>
        | '(' <expr> ')'
    }
    
    token func_name { 'print' }
    
    rule stmt {
        | <let_expr>
        | <assignment>
        | <func_call>
        | <comment>
    }
    
    rule let_expr {
        'let' <var_name> [ '=' <value> ]?
    }
    
    rule assignment {
        <var_name> '=' <value>
    }
    
    rule func_call {
        <func_name> '(' <var_name> ')'
    }
    
    rule comment {
        '//' \N* \n?
    }
}
