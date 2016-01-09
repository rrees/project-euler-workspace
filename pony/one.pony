use "collections"

actor Main
  new create(env: Env) =>

    let ns = Range(1,1000)
    var sum: Integer[U64] = 0


    for n in ns do
      if ((n % 3) == 0) or ((n % 5) == 0) then
        sum = sum + n
      end
    end

    env.out.print(sum.string())
