require 'open3'
require 'digest/md5'

TSUNAMI_PATH = "tsunami"
TSUNAMID_PATH = "tsunamid"
NAMIDA_PATH = ["../target/release/namida"]
#NAMIDA_PATH = ["cargo", "miri", "run"]

puts "namida client, namida server"
puts "----------------------------"
puts

start = Time.now
sin, sout, swait = Open3.popen2e({ "RUST_BACKTRACE" => "1" }, *NAMIDA_PATH, "serve", "--verbose", "--transcript", "source/fish.jpg", "--secret", "psk.txt")
cin, cout, cwait = Open3.popen2e({ "RUST_BACKTRACE" => "1" }, *NAMIDA_PATH, "get", "--secret", "psk.txt", "--transcript", "--server", "127.0.0.1", "source/fish.jpg")

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

if File.exist?("fish.jpg")
  content = File.read("fish.jpg")
  digest = Digest::MD5.hexdigest(content)
  if digest == "17f6d0c96590ad1c933314c0cbdb0aa0"
    if Dir["*.namc"].empty?
      puts "not ok, missing client transcript"
    elsif Dir["*.nams"].empty?
        puts "not ok, missing client transcript"
    else
      puts "ok"
      (Dir["*.namc"] + Dir["*.nams"]).each do |file|
        File.delete(file)
      end
    end
  else
    puts "not ok, read #{content.length} bytes, md5: #{digest}"
  end
  File.delete("fish.jpg")
else
  puts "not ok, no file created"
end

puts "time taken: #{finish - start}"
