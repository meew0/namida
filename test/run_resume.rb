require 'open3'
require 'digest/md5'

NAMIDA_PATH = ["../target/release/namida"]

puts "-- Resume test --"

data = Random.new.bytes(1000000000)
File.write('source/100M.bin', data)
source_digest = Digest::MD5.hexdigest(data)
20000000.upto(40000000) { |i| data[i] = 'a' }
puts "Source data MD5: #{source_digest}, after damage: #{Digest::MD5.hexdigest(data)}"
File.write('100M.bin', data)

start = Time.now
sin, sout, swait = Open3.popen2e({ "RUST_BACKTRACE" => "1" }, *NAMIDA_PATH, "serve", "--verbose", "--transcript", "source/100M.bin", "--secret", "psk.txt")
cin, cout, cwait = Open3.popen2e({ "RUST_BACKTRACE" => "1" }, *NAMIDA_PATH, "get", "--secret", "psk.txt", "--transcript", "--server", "127.0.0.1", "source/100M.bin")

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

if File.exist?("100M.bin")
  content = File.read("100M.bin")
  digest = Digest::MD5.hexdigest(content)
  if digest == source_digest
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
  File.delete("100M.bin")
  File.delete("source/100M.bin")
else
  puts "not ok, no file created"
end

puts "time taken: #{finish - start}"
