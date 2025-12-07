use lib 'lib';
use Edge :ALL;

my $input = 'test.ed'.IO.slurp();
my $match_result = EdgeGram.parse($input, actions => EdgeActions) orelse {
    my $exception = .exception;

    # Properties available:
    my $line_num = $exception.line;
    my $col_num = $exception.column;
	my $msg = $exception.message;
	my @lines = $msg
		andthen .match: /^ (\n)+ $/,:ex
		andthen .map: *.[0].put;
	say @lines;

    exit;
};

