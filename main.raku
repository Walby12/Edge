use lib 'lib';
use Edge;

my $code = 'test.ed'.IO.slurp();
my $result = EdgeGram.parse($code, :actions(EdgeActions));
# say $result 
