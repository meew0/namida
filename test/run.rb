require 'open3'
require 'digest/md5'

TSUNAMI_PATH = "tsunami"
TSUNAMID_PATH = "tsunamid"
NAMIDA_PATH = ["../target/debug/namida"]
#NAMIDA_PATH = ["cargo", "miri", "run"]

puts "namida client, namida server"
puts "----------------------------"
puts

sin, sout, swait = Open3.popen2e(*NAMIDA_PATH, "server", "source/fish.jpg")
cin, cout, cwait = Open3.popen2e(*NAMIDA_PATH, "client")

sleep 0.1
cin.puts "connect 127.0.0.1"
sleep 0.1
cin.puts "dir"
sleep 0.2
cin.puts "get source/fish.jpg"
sleep 0.5

Process.kill("KILL", cwait.pid) rescue nil
Process.kill("KILL", swait.pid) rescue nil

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

if File.exist?("fish.jpg")
  content = File.read("fish.jpg")
  digest = Digest::MD5.hexdigest(content)
  if digest == "17f6d0c96590ad1c933314c0cbdb0aa0"
    puts "ok"
  else
    puts "not ok, read #{content.length} bytes, md5: #{digest}"
  end
  File.delete("fish.jpg")
else
  puts "not ok, no file created"
end
