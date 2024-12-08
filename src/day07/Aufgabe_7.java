package day07;

import java.io.IOException;
import java.nio.file.Files;
import java.nio.file.Path;
import java.util.ArrayList;
import java.util.List;
import java.util.regex.Matcher;
import java.util.regex.Pattern;
import java.util.stream.Collectors;
import java.util.stream.Stream;

public class Aufgabe_7 {

	private static List<String> inputList = new ArrayList<>();
	private static List<String> inputList2 = new ArrayList<>();

	private static Pattern pattern1 = Pattern.compile("\\d{1,20}");
	private static Pattern pattern2 = Pattern.compile(" ");
	private static Matcher matcher1;
	private static Matcher matcher2;

	public static void main(String[] args) {
		Path path = Path.of("src/day07/Task07.txt");

		try (Stream<String> input = Files.lines(path); Stream<String> input2 = Files.lines(path)) {
			inputList = input.collect(Collectors.toList());
			inputList2 = input2.collect(Collectors.toList());

			long result1 = 0L;

			for (String line : inputList) {
				int amountOfSpace = 0;
				matcher2 = pattern2.matcher(line);
				while (matcher2.find()) {
					amountOfSpace++;
				}

				matcher1 = pattern1.matcher(line);
				int calcAdd = 0;
				int calcMul = 1;

//				if (matcher1.find()) {
//					r = Integer.valueOf(matcher1.group());
//				}

//				while (matcher1.find()) {
//					calcAdd += Integer.valueOf(matcher1.group());
//					calcMul *= Integer.valueOf(matcher1.group());
//					if (calcAdd == r) {
//						result1 += calcAdd;
//					}
//					if (calcMul == r) {
//						result1 += calcMul;
//					}

			}

		}

		System.out.println(result1);

	}catch(

	IOException e)
	{
		System.out.println("Error");
	}

	}

	private static long addAll(String line) {
		matcher1 = pattern1.matcher(line);
		int r = 0;
		long calcAdd = 0;

		if (matcher1.find()) {
			r = Integer.valueOf(matcher1.group());
		}

		while (matcher1.find()) {
			calcAdd += Integer.valueOf(matcher1.group());
			if (calcAdd == r) {
				return calcAdd;
			}
		}
		return 0;
	}
}
