puts "Cache-Control: no-cache\n";
puts "Content-type: text/html\n\n";

puts "<html><head><title>Hello CGI World</title></head>";
puts "<body><h1 align=center>Hello CGI World!</h1><hr/>\n";

puts "Hello World<br/>\n";
puts "This page was generated by the Ruby programming language<br/>\n";
puts "This program was generated at: #{Time.now}<br/>\n";
puts "Your current IP address is: #{ENV['REMOTE_ADDR']}<br/>\n";

puts "</body></html>\n";
