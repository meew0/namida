require 'open3'
require 'digest/md5'
require 'fileutils'

NAMIDA_PATH = [File.join(__dir__, "../target/debug/namida")]

puts "namida client, namida server"
puts "----------------------------"
puts

FileUtils.mkdir_p 'temp'

start = Time.now
sin, sout, swait = Open3.popen2e({ "RUST_BACKTRACE" => "1" }, *NAMIDA_PATH, "serve", "--verbose", "--secret", "psk.txt", chdir: "issue_4")
cin, cout, cwait = Open3.popen2e({ "RUST_BACKTRACE" => "1" }, *NAMIDA_PATH, "get", "--secret", "psk.txt", "--server", "127.0.0.1", "--all", chdir: "temp")

sleep 1.0

Process.kill("KILL", cwait.pid) rescue puts "failed to kill client"
Process.kill("KILL", swait.pid) rescue puts "failed to kill server"
finish = Time.now

puts "client output:"
puts cout.read
puts

puts "server output:"
puts sout.read
puts

sin.close
sout.close
cin.close
cout.close

puts "example1 exists: #{File.exist?("temp/test1/example.txt")}"
puts "example2 exists: #{File.exist?("temp/test2/example.txt")}"

FileUtils.rm_r 'temp'

puts "time taken: #{finish - start}"
