package nn.datasets;

import nn.NeuralNetwork;
import nn.Pair;
import nn.TrainingSet;
import nn.Utility;

import java.io.*;
import java.nio.charset.StandardCharsets;
import java.util.*;
import java.util.stream.Collectors;
import java.util.stream.Stream;

import static nn.Utility.color.*;

public class AllColors {
    public static int testCount = 100; // how large each dataset is
    public static String fileName = "data3.txt";
    public static int input_size = 4;
    public static int output_size = 7;

    public static void main(String[] args) throws IOException {
        TrainingSet traningSet = getData_allColors(testCount, fileName).first;
        NeuralNetwork nn = new NeuralNetwork(new ArrayList<>(Arrays.asList(input_size, 74, 89, output_size)), 0.000056, 123456);
        nn.predictTime(new double[]{42, 42, 42, 42});
        nn.fit(traningSet.tasks, traningSet.targets, 100, true, 0, true, 11);

        Map<double[], Utility.color> testSet = Stream.of(new Object[][]{
                {new double[]{65, 122, 72, 217}, BLACK},
                {new double[]{109, 58, 36, 167}, RED},
                {new double[]{18, 41, 34, 95}, NOTHING},
                //{new double[]{13, 29, 22}, UNKNOWN},
        }).collect(Collectors.toMap(data -> (double[]) data[0], data -> (Utility.color) data[1]));

        testSet.forEach((input, expected) -> check_all_colors(input, nn, expected));
        Utility.printNN(nn);
    }

    public static void check_all_colors(double[] input, NeuralNetwork nn, Utility.color expected) {
        Utility.color output = UNKNOWN;
        List<Double> prediction = nn.predict(input);
        switch (prediction.indexOf(Collections.max(prediction))) {
            case 0 -> output = BLACK;
            case 1 -> output = WHITE;
            case 2 -> output = BLUE;
            case 3 -> output = GREEN;
            case 4 -> output = YELLOW;
            case 5 -> output = RED;
            case 6 -> output = NOTHING;
        }
        if (output != expected) {
            System.out.println("FAILED");
            System.out.println("expected: " + expected);
            System.out.println("output: " + output);
            System.out.println("prediction: " + prediction);
        } else {
            System.out.println("Success! " + output);
        }
        System.out.println();
    }

    public static double[][][] getData(String fileName, int testCount) {
        try {
            // File path is passed as parameter
            InputStream is = AllColors.class.getClassLoader().getResourceAsStream(fileName);

            InputStreamReader streamReader =
                    new InputStreamReader(is, StandardCharsets.UTF_8);

            // Note:  Double backquote is to avoid compiler
            // interpret words
            // like \test as \t (ie. as a escape sequence)

            // Creating an object of BufferedReader class
            BufferedReader br
                    = new BufferedReader(streamReader);

            // Declaring a string variable
            String st;
            // Condition holds true till
            // there is character in a string
            StringBuilder sb = new StringBuilder();
            while ((st = br.readLine()) != null) {
                sb.append(st.replace("\n", ""));
            }
            String content = sb.toString();
//            System.out.println(content);
            content = content.replace("{{{", "{{").replace("}}}", "}}").replace("}},{{", "},{");
//            System.out.println(content);
            String[][] arr = Arrays.stream(content.substring(2, content.length() - 2).split("\\},\\{"))
                    .map(e -> Arrays.stream(e.split("\\s*,\\s*"))
                            .toArray(String[]::new)).toArray(String[][]::new);
            double[][] double_arr = new double[arr.length][arr[0].length];
//            System.out.println(Arrays.deepToString(arr));
            for (int i = 0; i < arr.length; i++) {
                for (int j = 0; j < arr[0].length; j++) {
                    double_arr[i][j] = Double.parseDouble(arr[i][j].replace("{", "").replace("}", ""));
                }
            }
            double[][][] X = new double[7][testCount][4];
            for (int i = 0; i < X.length; i++) {
                for (int j = 0; j < X[0].length; j++) {
                    System.arraycopy(double_arr[i * X[0].length + j], 0, X[i][j], 0, X[0][0].length);
                }
            }
//            System.out.println(Arrays.deepToString(X).replace("[", "{").replace("]", "}").replace(".0", "").replace(" ", ""));
//            System.out.printf("X.len: %d\nX[0].len: %d\nX[0][0].len: %d\n", X.length, X[0].length, X[0][0].length);
            return X;
        } catch (Exception e) {
            e.printStackTrace();
        }
        return null;
    }

    public static Pair<TrainingSet, TrainingSet> getData_allColors(int testCount, String fileName) throws IOException {
//        double[][][] X = {{{47, 89, 61, 194}, {65, 122, 72, 217}, {36, 67, 48, 142}, {36, 70, 50, 148}, {36, 65, 49, 147}, {112, 186, 94, 265}, {18, 40, 32, 91}, {20, 43, 35, 99}, {70, 122, 80, 250}, {71, 127, 113, 346}, {172, 293, 145, 415}, {34, 66, 49, 138}, {18, 40, 32, 92}, {19, 41, 34, 95}, {18, 39, 32, 90}, {16, 39, 32, 90}, {32, 64, 47, 131}, {17, 38, 32, 90}, {25, 50, 39, 110}, {21, 43, 35, 99}, {16, 37, 30, 83}, {80, 146, 103, 324}, {21, 44, 36, 102}, {17, 40, 33, 93}, {23, 48, 38, 109}, {19, 41, 34, 95}, {17, 38, 30, 84}, {17, 38, 32, 91}, {22, 47, 36, 100}, {31, 55, 41, 118}, {19, 40, 33, 92}, {20, 42, 33, 95}, {98, 168, 76, 245}, {25, 46, 33, 97}, {18, 39, 32, 90}, {17, 39, 33, 92}, {17, 37, 29, 84}, {18, 40, 33, 93}, {19, 40, 33, 93}, {68, 112, 68, 232}, {18, 39, 32, 90}, {20, 43, 36, 99}, {27, 47, 37, 109}, {68, 120, 113, 332}, {18, 40, 31, 88}, {26, 47, 36, 104}, {18, 39, 32, 89}, {17, 40, 33, 92}, {19, 40, 32, 91}, {49, 82, 55, 173}, {17, 39, 31, 86}, {18, 40, 33, 92}, {18, 39, 32, 89}, {101, 173, 154, 443}, {20, 44, 34, 97}, {78, 140, 100, 330}, {33, 63, 46, 134}, {189, 297, 129, 375}, {37, 67, 50, 149}, {89, 159, 112, 366}, {29, 55, 42, 123}, {35, 55, 40, 129}, {54, 92, 63, 197}, {222, 360, 136, 383}, {18, 39, 31, 88}, {24, 49, 40, 112}, {57, 97, 66, 206}, {71, 127, 90, 296}, {40, 71, 52, 160}, {69, 124, 83, 278}, {25, 50, 39, 112}, {106, 186, 121, 365}, {29, 55, 42, 123}, {33, 62, 45, 133}, {25, 50, 39, 111}, {34, 64, 46, 136}, {25, 49, 38, 111}, {19, 42, 35, 97}, {34, 62, 48, 141}, {28, 55, 43, 122}, {50, 86, 60, 190}, {27, 57, 44, 130}, {30, 57, 44, 129}, {45, 62, 43, 145}, {19, 40, 33, 92}, {29, 56, 44, 128}, {16, 36, 29, 82}, {17, 38, 32, 89}, {19, 43, 34, 96}, {174, 298, 127, 354}, {20, 44, 35, 98}, {33, 64, 45, 135}, {22, 42, 34, 97}, {22, 46, 37, 106}, {17, 39, 31, 87}, {39, 69, 52, 162}, {24, 48, 38, 107}, {203, 340, 130, 348}, {20, 43, 34, 96}, {74, 126, 78, 241}, {18, 39, 31, 89}, {89, 159, 142, 440}, {20, 42, 34, 96}, {45, 62, 44, 145}, {17, 40, 32, 89}, {95, 162, 146, 433}, {17, 38, 32, 87}, {17, 40, 34, 92}, {17, 38, 30, 86}, {16, 39, 31, 89}, {21, 45, 36, 101}, {56, 85, 51, 166}, {17, 39, 31, 88}, {45, 78, 54, 168}, {17, 38, 31, 87}, {17, 39, 33, 91}, {52, 89, 63, 193}, {86, 145, 94, 269}, {40, 71, 52, 159}, {52, 87, 58, 183}, {30, 57, 44, 127}, {22, 46, 37, 106}, {29, 56, 43, 125}, {82, 139, 105, 344}, {25, 50, 38, 110}, {45, 78, 52, 165}, {18, 41, 33, 91}, {18, 40, 33, 92}, {65, 117, 78, 227}, {106, 180, 120, 345}, {68, 113, 72, 229}, {32, 61, 47, 142}, {39, 72, 52, 155}, {47, 82, 55, 168}, {28, 57, 42, 123}, {37, 68, 49, 150}, {46, 84, 57, 174}, {22, 47, 37, 107}, {30, 59, 43, 124}, {70, 121, 73, 238}, {36, 66, 47, 139}, {26, 53, 41, 118}, {17, 39, 31, 88}, {18, 41, 34, 94}, {222, 363, 153, 435}, {29, 59, 45, 124}, {142, 241, 177, 569}, {135, 228, 148, 425}, {72, 124, 77, 244}, {105, 185, 145, 444}, {19, 41, 33, 94}, {20, 45, 37, 102}, {26, 53, 41, 119}, {68, 101, 57, 194}, {27, 53, 41, 118}, {50, 85, 56, 175}, {19, 43, 34, 95}, {19, 42, 34, 94}, {20, 42, 33, 92}, {130, 213, 115, 329}, {95, 153, 92, 299}, {97, 169, 144, 426}, {45, 80, 58, 175}, {73, 132, 88, 297}, {19, 41, 32, 96}, {20, 44, 35, 99}, {19, 42, 33, 93}, {19, 41, 33, 94}, {78, 132, 81, 262}, {91, 154, 96, 273}, {34, 63, 47, 140}, {92, 161, 129, 414}, {30, 59, 43, 127}, {65, 117, 77, 246}, {22, 47, 35, 100}, {32, 62, 46, 134}, {20, 44, 35, 96}, {19, 43, 35, 97}, {26, 52, 38, 112}, {32, 61, 48, 142}, {29, 59, 43, 124}, {27, 55, 43, 125}, {34, 64, 46, 135}, {55, 102, 65, 205}, {109, 183, 119, 393}, {70, 119, 72, 231}, {27, 55, 41, 121}, {53, 97, 65, 199}, {46, 81, 57, 178}, {51, 86, 58, 183}, {35, 64, 48, 144}, {35, 67, 47, 143}, {33, 62, 48, 143}, {26, 52, 42, 119}, {35, 66, 48, 145}, {33, 54, 40, 124}, {31, 59, 45, 131}, {45, 78, 53, 166}, {25, 50, 37, 108}, {59, 98, 60, 196}}, {{690, 1115, 590, 1948}, {584, 935, 508, 1653}, {691, 1116, 590, 1950}, {583, 934, 506, 1650}, {679, 1099, 576, 1899}, {562, 901, 454, 1479}, {549, 882, 466, 1507}, {587, 941, 483, 1581}, {467, 740, 412, 1331}, {523, 834, 426, 1377}, {396, 623, 358, 1164}, {482, 767, 403, 1310}, {443, 704, 392, 1261}, {508, 805, 416, 1347}, {450, 715, 396, 1279}, {512, 817, 417, 1342}, {543, 870, 457, 1481}, {460, 728, 384, 1237}, {389, 618, 349, 1124}, {464, 740, 388, 1249}, {467, 743, 403, 1308}, {357, 571, 272, 880}, {419, 665, 375, 1207}, {456, 723, 383, 1238}, {310, 492, 287, 914}, {403, 643, 349, 1109}, {233, 378, 200, 646}, {318, 513, 256, 823}, {465, 752, 411, 1323}, {490, 783, 396, 1279}, {443, 706, 393, 1268}, {420, 669, 359, 1153}, {451, 715, 393, 1280}, {67, 110, 78, 277}, {362, 576, 326, 1050}, {454, 726, 372, 1198}, {389, 620, 344, 1107}, {150, 208, 119, 400}, {366, 587, 328, 1048}, {313, 501, 256, 831}, {641, 1023, 540, 1778}, {658, 1053, 546, 1724}, {417, 673, 372, 1204}, {256, 402, 252, 829}, {274, 439, 259, 832}, {97, 156, 102, 335}, {346, 550, 318, 1021}, {443, 711, 396, 1291}, {239, 380, 229, 735}, {273, 435, 251, 803}, {117, 189, 124, 396}, {79, 130, 84, 267}, {252, 395, 244, 786}, {86, 143, 99, 312}, {240, 377, 251, 820}, {355, 563, 296, 958}, {232, 368, 224, 718}, {216, 349, 207, 664}, {79, 129, 89, 278}, {102, 166, 115, 373}, {129, 207, 136, 430}, {64, 110, 78, 244}, {164, 262, 166, 529}, {137, 204, 120, 387}, {67, 93, 59, 196}, {207, 333, 194, 606}, {233, 369, 224, 721}, {282, 448, 260, 830}, {141, 225, 153, 487}, {56, 93, 63, 200}, {97, 154, 101, 319}, {22, 45, 37, 105}, {190, 302, 189, 602}, {179, 286, 190, 616}, {200, 310, 197, 635}, {343, 539, 298, 965}, {192, 306, 187, 599}, {180, 270, 164, 521}, {369, 579, 337, 1246}, {52, 91, 61, 190}, {258, 402, 239, 771}, {223, 345, 227, 752}, {210, 332, 204, 655}, {266, 424, 246, 787}, {392, 623, 353, 1136}, {230, 365, 217, 693}, {224, 356, 217, 698}, {144, 230, 162, 535}, {269, 427, 254, 816}, {351, 559, 309, 992}, {290, 460, 271, 873}, {409, 651, 352, 1130}, {447, 711, 393, 1274}, {411, 655, 340, 1121}, {593, 955, 514, 1639}, {255, 411, 233, 753}, {495, 792, 431, 1390}, {274, 436, 236, 777}, {256, 407, 244, 784}, {23, 47, 39, 111}, {319, 496, 296, 961}, {157, 246, 170, 564}, {290, 453, 271, 873}, {235, 374, 219, 704}, {275, 434, 258, 830}, {126, 200, 135, 443}, {211, 334, 214, 682}, {132, 210, 143, 473}, {187, 291, 183, 583}, {219, 351, 208, 665}, {110, 179, 121, 381}, {54, 95, 68, 192}, {37, 67, 50, 152}, {157, 250, 233, 752}, {153, 243, 159, 507}, {164, 263, 162, 519}, {160, 255, 163, 519}, {141, 228, 145, 461}, {134, 214, 145, 472}, {127, 206, 141, 456}, {114, 186, 119, 380}, {45, 78, 56, 177}, {133, 207, 133, 429}, {38, 60, 46, 142}, {297, 469, 278, 895}, {329, 519, 293, 949}, {379, 591, 339, 1109}, {321, 517, 285, 908}, {419, 660, 372, 1175}, {434, 706, 407, 1224}, {563, 910, 478, 1544}, {159, 242, 114, 363}, {196, 316, 189, 637}, {355, 567, 310, 988}, {406, 646, 385, 1231}, {332, 530, 287, 909}, {343, 534, 312, 1019}, {351, 553, 309, 1001}, {375, 592, 340, 1099}, {295, 474, 264, 848}, {238, 373, 247, 809}, {177, 283, 197, 645}, {435, 691, 386, 1245}, {319, 510, 283, 911}, {259, 411, 246, 791}, {194, 312, 189, 605}, {38, 68, 52, 156}, {20, 43, 35, 102}, {86, 142, 95, 296}, {24, 50, 39, 115}, {115, 183, 121, 387}, {105, 169, 115, 367}, {151, 245, 154, 488}, {106, 173, 113, 360}, {177, 284, 175, 558}, {183, 294, 178, 571}, {203, 322, 200, 638}, {223, 357, 212, 678}, {54, 90, 63, 186}, {20, 44, 36, 103}, {141, 222, 149, 475}, {219, 346, 209, 674}, {103, 159, 87, 276}, {266, 423, 243, 777}, {228, 362, 222, 709}, {191, 306, 192, 614}, {291, 462, 273, 879}, {122, 197, 136, 433}, {225, 356, 219, 700}, {237, 380, 222, 712}, {307, 486, 283, 913}, {270, 432, 247, 769}, {133, 213, 143, 449}, {89, 145, 97, 308}, {155, 249, 160, 505}, {127, 209, 132, 417}, {193, 306, 190, 608}, {199, 319, 191, 613}, {223, 354, 218, 696}, {130, 209, 146, 476}, {233, 369, 226, 725}, {87, 140, 86, 280}, {214, 339, 214, 681}, {84, 135, 84, 272}, {136, 215, 138, 432}, {57, 99, 71, 222}, {142, 210, 128, 431}, {149, 232, 152, 516}, {217, 344, 213, 679}, {237, 380, 223, 714}, {243, 382, 235, 755}, {269, 429, 246, 790}, {226, 359, 218, 700}, {280, 450, 246, 779}, {233, 369, 222, 716}, {281, 465, 263, 842}, {138, 220, 149, 474}, {119, 188, 125, 406}, {136, 217, 143, 451}, {204, 326, 195, 622}}, {{100, 240, 254, 536}, {79, 211, 228, 463}, {69, 194, 233, 457}, {51, 168, 208, 382}, {36, 114, 150, 284}, {40, 137, 178, 324}, {36, 101, 125, 271}, {44, 130, 157, 301}, {49, 78, 67, 180}, {34, 116, 151, 277}, {32, 102, 122, 233}, {31, 111, 147, 266}, {115, 271, 277, 595}, {37, 63, 58, 157}, {52, 180, 231, 423}, {45, 157, 201, 360}, {41, 124, 159, 306}, {30, 101, 131, 246}, {38, 97, 124, 257}, {28, 88, 119, 229}, {50, 125, 153, 321}, {35, 115, 151, 282}, {46, 89, 82, 196}, {36, 118, 152, 283}, {34, 116, 155, 287}, {32, 109, 143, 267}, {104, 237, 245, 505}, {25, 74, 101, 202}, {26, 83, 97, 198}, {26, 92, 117, 224}, {33, 118, 161, 294}, {78, 190, 194, 407}, {70, 162, 177, 389}, {44, 122, 145, 296}, {42, 106, 129, 272}, {46, 111, 126, 275}, {35, 99, 124, 244}, {25, 82, 105, 207}, {35, 109, 142, 272}, {26, 75, 105, 207}, {27, 85, 109, 213}, {31, 74, 85, 192}, {25, 78, 100, 197}, {75, 155, 160, 364}, {29, 91, 121, 232}, {20, 55, 65, 141}, {131, 274, 240, 550}, {24, 80, 106, 205}, {57, 133, 150, 332}, {26, 85, 111, 212}, {20, 49, 51, 119}, {18, 42, 37, 99}, {18, 43, 39, 101}, {20, 54, 58, 131}, {19, 42, 36, 97}, {24, 62, 71, 159}, {115, 235, 238, 585}, {70, 157, 164, 359}, {42, 90, 100, 220}, {60, 150, 167, 348}, {52, 121, 125, 272}, {84, 168, 149, 351}, {40, 74, 68, 169}, {58, 123, 123, 287}, {37, 67, 56, 149}, {52, 120, 134, 296}, {27, 85, 109, 214}, {38, 97, 121, 254}, {117, 246, 208, 463}, {28, 66, 72, 167}, {29, 86, 117, 227}, {28, 86, 115, 225}, {22, 67, 82, 176}, {23, 73, 92, 184}, {35, 111, 143, 274}, {42, 148, 189, 344}, {144, 322, 321, 742}, {75, 207, 228, 447}, {55, 137, 161, 339}, {56, 140, 161, 337}, {44, 98, 111, 244}, {45, 105, 115, 257}, {59, 128, 129, 302}, {66, 142, 125, 279}, {21, 59, 70, 146}, {22, 65, 80, 164}, {29, 84, 111, 222}, {83, 163, 142, 337}, {98, 201, 188, 441}, {77, 172, 177, 430}, {23, 65, 76, 160}, {25, 70, 86, 175}, {19, 49, 52, 120}, {20, 53, 61, 136}, {19, 51, 56, 125}, {22, 62, 78, 162}, {35, 84, 95, 208}, {23, 71, 90, 181}, {37, 114, 148, 282}, {42, 119, 149, 295}, {96, 202, 197, 440}, {44, 121, 151, 301}, {59, 150, 178, 370}, {63, 154, 171, 357}, {40, 125, 161, 308}, {43, 134, 171, 325}, {56, 174, 221, 415}, {49, 159, 200, 369}, {79, 227, 260, 507}, {78, 217, 238, 472}, {124, 297, 301, 661}, {47, 162, 220, 396}, {53, 186, 237, 429}, {64, 198, 231, 443}, {55, 183, 244, 444}, {55, 169, 223, 408}, {63, 199, 244, 457}, {50, 172, 217, 391}, {35, 130, 174, 314}, {36, 136, 181, 321}, {31, 119, 155, 276}, {33, 120, 154, 278}, {30, 105, 137, 251}, {27, 89, 114, 218}, {38, 116, 136, 496}, {52, 160, 189, 356}, {146, 312, 304, 718}, {158, 343, 263, 542}, {50, 172, 226, 409}, {41, 151, 193, 345}, {37, 144, 199, 351}, {38, 145, 191, 338}, {31, 105, 141, 266}, {32, 116, 153, 276}, {52, 145, 180, 355}, {45, 145, 185, 344}, {48, 160, 196, 363}, {184, 361, 250, 526}, {114, 240, 227, 492}, {136, 300, 312, 701}, {71, 188, 217, 429}, {90, 235, 247, 531}, {43, 165, 225, 394}, {37, 133, 178, 318}, {38, 144, 190, 339}, {38, 140, 188, 333}, {264, 525, 377, 816}, {245, 467, 305, 638}, {82, 238, 273, 519}, {62, 172, 199, 373}, {48, 165, 217, 395}, {33, 127, 204, 376}, {28, 121, 163, 284}, {43, 156, 204, 362}, {32, 115, 156, 286}, {40, 150, 194, 343}, {51, 166, 213, 396}, {44, 157, 202, 360}, {46, 160, 213, 386}, {44, 161, 205, 365}, {32, 102, 135, 255}, {49, 158, 195, 367}, {47, 160, 210, 385}, {31, 101, 131, 249}, {27, 83, 115, 226}, {23, 64, 80, 171}, {45, 154, 193, 355}, {39, 123, 144, 278}, {38, 135, 202, 374}, {48, 161, 199, 361}, {42, 154, 195, 350}, {43, 117, 129, 264}, {40, 135, 180, 333}, {34, 96, 120, 241}, {35, 127, 171, 311}, {41, 143, 187, 339}, {21, 56, 60, 135}, {25, 73, 102, 202}, {58, 130, 140, 329}, {91, 200, 195, 467}, {29, 92, 121, 234}, {27, 102, 126, 0}, {49, 178, 230, 414}, {92, 243, 267, 568}, {53, 186, 238, 430}, {45, 156, 197, 357}, {45, 167, 236, 415}, {42, 143, 223, 402}, {39, 142, 177, 321}, {35, 127, 171, 306}, {31, 109, 140, 266}, {36, 128, 163, 294}, {157, 345, 312, 654}, {25, 83, 100, 194}, {29, 93, 125, 235}, {20, 54, 55, 124}, {28, 100, 138, 257}, {22, 71, 95, 188}, {152, 295, 239, 563}, {22, 69, 85, 170}}, {{46, 194, 78, 267}, {123, 318, 131, 419}, {45, 195, 78, 269}, {69, 223, 104, 356}, {45, 187, 77, 259}, {37, 150, 66, 215}, {74, 237, 99, 338}, {57, 192, 80, 263}, {38, 175, 71, 238}, {33, 155, 63, 206}, {43, 135, 69, 224}, {34, 132, 63, 204}, {30, 135, 59, 191}, {32, 150, 62, 201}, {68, 159, 83, 280}, {31, 111, 52, 164}, {40, 160, 70, 231}, {63, 170, 80, 268}, {42, 148, 70, 230}, {34, 74, 50, 159}, {28, 122, 56, 184}, {31, 130, 60, 192}, {27, 113, 52, 165}, {25, 94, 49, 153}, {31, 122, 58, 188}, {33, 126, 58, 183}, {25, 111, 52, 167}, {92, 186, 118, 379}, {31, 108, 55, 178}, {29, 112, 53, 170}, {33, 137, 61, 201}, {32, 135, 61, 196}, {26, 103, 51, 164}, {27, 107, 53, 168}, {25, 104, 50, 161}, {24, 98, 48, 152}, {22, 75, 44, 133}, {15, 42, 31, 92}, {36, 109, 61, 190}, {91, 247, 121, 426}, {29, 119, 56, 183}, {27, 113, 53, 169}, {29, 111, 53, 171}, {30, 124, 57, 184}, {35, 115, 60, 191}, {54, 121, 69, 226}, {143, 309, 194, 612}, {21, 67, 41, 119}, {20, 81, 41, 132}, {27, 112, 54, 172}, {25, 94, 49, 155}, {27, 95, 51, 160}, {112, 259, 128, 428}, {50, 134, 68, 230}, {27, 126, 52, 172}, {28, 120, 53, 172}, {28, 115, 55, 176}, {29, 116, 56, 178}, {26, 103, 51, 164}, {56, 128, 70, 236}, {26, 95, 50, 158}, {48, 121, 68, 226}, {58, 143, 78, 261}, {30, 80, 51, 156}, {27, 105, 51, 164}, {22, 75, 43, 131}, {23, 89, 48, 148}, {39, 109, 61, 197}, {38, 178, 70, 235}, {42, 116, 62, 204}, {83, 248, 117, 361}, {180, 367, 146, 441}, {31, 140, 58, 189}, {26, 105, 50, 158}, {26, 109, 51, 165}, {23, 101, 48, 155}, {32, 128, 59, 192}, {29, 115, 55, 176}, {33, 119, 62, 193}, {37, 128, 64, 205}, {26, 98, 51, 155}, {23, 83, 45, 142}, {30, 98, 54, 171}, {29, 113, 54, 174}, {27, 90, 51, 160}, {32, 87, 55, 174}, {33, 108, 57, 180}, {74, 172, 81, 271}, {48, 147, 74, 245}, {100, 257, 126, 432}, {27, 101, 53, 165}, {61, 112, 64, 217}, {20, 65, 39, 120}, {20, 66, 42, 125}, {20, 65, 40, 120}, {18, 50, 36, 105}, {25, 93, 49, 155}, {22, 76, 45, 140}, {26, 91, 50, 159}, {36, 90, 58, 186}, {32, 127, 61, 196}, {38, 128, 63, 206}, {48, 147, 74, 244}, {153, 337, 243, 701}, {25, 90, 48, 148}, {60, 164, 83, 258}, {26, 104, 49, 156}, {25, 102, 49, 154}, {27, 118, 54, 178}, {22, 70, 45, 138}, {26, 116, 52, 171}, {20, 43, 35, 101}, {23, 81, 45, 140}, {28, 89, 50, 156}, {22, 68, 42, 126}, {29, 110, 55, 175}, {28, 121, 57, 183}, {33, 79, 53, 160}, {17, 51, 34, 100}, {19, 52, 36, 107}, {22, 79, 45, 139}, {29, 101, 53, 166}, {26, 102, 50, 161}, {30, 118, 57, 178}, {126, 273, 132, 391}, {24, 90, 48, 148}, {23, 88, 47, 147}, {22, 85, 45, 140}, {22, 77, 44, 137}, {25, 78, 48, 149}, {24, 95, 47, 150}, {25, 92, 48, 154}, {26, 107, 49, 160}, {33, 142, 61, 198}, {35, 166, 67, 220}, {33, 150, 62, 204}, {42, 183, 72, 243}, {85, 248, 123, 427}, {205, 442, 165, 529}, {216, 442, 160, 496}, {72, 239, 105, 337}, {113, 282, 145, 442}, {73, 243, 107, 347}, {46, 176, 77, 250}, {30, 132, 56, 183}, {28, 115, 52, 166}, {26, 115, 50, 161}, {29, 137, 57, 186}, {31, 145, 59, 197}, {35, 163, 66, 219}, {38, 176, 70, 234}, {148, 345, 120, 387}, {90, 274, 124, 425}, {122, 287, 117, 386}, {96, 284, 133, 429}, {75, 233, 110, 344}, {29, 129, 54, 179}, {19, 67, 37, 113}, {20, 72, 40, 127}, {24, 96, 49, 154}, {29, 126, 55, 181}, {18, 64, 36, 111}, {41, 196, 76, 257}, {34, 166, 67, 223}, {77, 243, 106, 364}, {82, 247, 114, 373}, {33, 154, 64, 213}, {31, 146, 61, 203}, {30, 138, 57, 192}, {27, 118, 51, 165}, {32, 151, 60, 201}, {26, 110, 50, 158}, {75, 241, 107, 307}, {257, 510, 176, 535}, {32, 148, 63, 206}, {45, 159, 71, 235}, {29, 111, 56, 176}, {28, 106, 53, 166}, {42, 118, 64, 207}, {49, 142, 70, 229}, {29, 103, 54, 169}, {90, 198, 96, 296}, {23, 88, 47, 146}, {19, 58, 37, 111}, {112, 241, 156, 495}, {37, 120, 63, 200}, {22, 96, 47, 151}, {22, 81, 46, 142}, {33, 140, 64, 208}, {27, 118, 52, 167}, {40, 188, 73, 245}, {202, 412, 167, 518}, {38, 178, 70, 233}, {43, 167, 73, 242}, {43, 143, 72, 233}, {70, 175, 98, 332}, {32, 117, 58, 185}, {73, 177, 84, 284}, {21, 71, 42, 124}, {17, 43, 34, 97}}, {{526, 651, 130, 970}, {540, 663, 149, 1009}, {512, 631, 124, 951}, {486, 580, 103, 834}, {396, 477, 100, 754}, {394, 472, 94, 717}, {448, 538, 106, 822}, {456, 548, 106, 810}, {452, 546, 116, 854}, {44, 67, 43, 150}, {368, 445, 106, 732}, {347, 419, 95, 669}, {421, 513, 116, 813}, {349, 426, 99, 677}, {370, 454, 96, 693}, {367, 444, 98, 697}, {353, 427, 91, 684}, {370, 445, 92, 687}, {399, 480, 103, 764}, {332, 400, 84, 633}, {440, 532, 119, 843}, {303, 368, 80, 564}, {237, 289, 76, 498}, {185, 231, 64, 402}, {421, 529, 148, 839}, {326, 434, 188, 867}, {395, 477, 98, 776}, {77, 104, 42, 210}, {301, 371, 77, 576}, {269, 326, 72, 525}, {319, 385, 85, 628}, {275, 337, 74, 532}, {303, 365, 88, 609}, {257, 313, 77, 519}, {285, 346, 85, 581}, {282, 339, 78, 554}, {288, 348, 79, 566}, {319, 385, 85, 609}, {283, 341, 78, 567}, {367, 445, 100, 701}, {329, 399, 102, 677}, {74, 100, 44, 204}, {439, 567, 205, 1035}, {85, 108, 45, 219}, {176, 216, 61, 407}, {98, 130, 47, 250}, {137, 174, 53, 316}, {249, 303, 69, 493}, {206, 251, 66, 437}, {218, 268, 64, 446}, {278, 337, 77, 557}, {162, 203, 61, 408}, {291, 353, 79, 578}, {280, 337, 73, 542}, {281, 339, 76, 551}, {272, 328, 73, 531}, {246, 304, 68, 463}, {258, 316, 70, 491}, {285, 346, 80, 572}, {283, 347, 85, 558}, {172, 216, 72, 428}, {233, 298, 88, 532}, {402, 512, 164, 885}, {563, 734, 174, 945}, {608, 795, 210, 1139}, {492, 614, 162, 945}, {152, 188, 61, 371}, {128, 164, 51, 300}, {183, 224, 62, 397}, {168, 209, 58, 367}, {205, 253, 76, 453}, {234, 288, 80, 497}, {226, 276, 72, 477}, {231, 289, 87, 520}, {306, 368, 83, 603}, {353, 430, 104, 684}, {398, 481, 106, 771}, {425, 540, 153, 916}, {538, 710, 201, 1037}, {531, 672, 182, 1011}, {415, 504, 107, 787}, {216, 267, 61, 405}, {279, 341, 73, 520}, {326, 394, 76, 566}, {338, 410, 87, 655}, {371, 447, 86, 670}, {375, 452, 97, 721}, {93, 127, 57, 245}, {370, 444, 95, 709}, {475, 578, 120, 861}, {452, 549, 108, 826}, {438, 531, 113, 804}, {432, 520, 102, 799}, {430, 523, 113, 796}, {467, 597, 185, 1022}, {122, 156, 55, 319}, {546, 684, 177, 1100}, {535, 718, 168, 808}, {473, 571, 107, 858}, {351, 427, 82, 600}, {453, 546, 104, 825}, {430, 515, 96, 753}, {350, 426, 89, 663}, {382, 458, 87, 687}, {351, 425, 88, 667}, {290, 352, 76, 552}, {524, 639, 120, 937}, {342, 418, 84, 606}, {441, 549, 111, 923}, {412, 506, 104, 845}, {562, 698, 154, 1042}, {610, 774, 214, 1184}, {584, 731, 168, 1100}, {531, 658, 153, 989}, {166, 204, 52, 317}, {190, 225, 53, 316}, {67, 96, 41, 195}, {137, 179, 57, 341}, {176, 216, 62, 482}, {116, 153, 45, 237}, {455, 551, 111, 842}, {388, 468, 86, 678}, {275, 661, 154, 1007}, {392, 474, 91, 706}, {403, 484, 100, 763}, {73, 103, 52, 214}, {297, 358, 87, 599}, {295, 358, 83, 580}, {246, 298, 75, 509}, {300, 369, 96, 612}, {149, 184, 57, 348}, {133, 168, 57, 348}, {44, 67, 38, 151}, {19, 42, 34, 96}, {162, 201, 59, 363}, {73, 99, 43, 195}, {169, 209, 59, 369}, {144, 185, 54, 331}, {173, 216, 60, 374}, {213, 264, 65, 434}, {273, 346, 115, 612}, {281, 341, 87, 572}, {123, 156, 51, 266}, {216, 263, 63, 439}, {120, 152, 50, 270}, {170, 211, 58, 369}, {134, 168, 54, 315}, {31, 54, 39, 125}, {141, 177, 58, 328}, {163, 207, 75, 405}, {171, 225, 74, 381}, {131, 167, 54, 319}, {109, 138, 47, 263}, {113, 146, 49, 281}, {129, 162, 51, 299}, {105, 130, 48, 262}, {37, 59, 35, 128}, {18, 41, 34, 94}, {126, 159, 52, 283}, {98, 128, 48, 261}, {121, 155, 53, 290}, {87, 117, 45, 207}, {79, 107, 44, 208}, {95, 127, 46, 238}, {132, 166, 57, 321}, {42, 66, 38, 143}, {90, 117, 46, 230}, {69, 96, 43, 199}, {145, 186, 67, 362}, {121, 155, 55, 313}, {84, 109, 47, 198}, {184, 227, 58, 382}, {111, 143, 49, 247}, {168, 211, 56, 335}, {121, 158, 53, 282}, {211, 256, 62, 426}, {210, 256, 65, 443}, {23, 46, 35, 105}, {81, 108, 46, 213}, {20, 42, 35, 98}, {106, 136, 49, 263}, {139, 175, 54, 317}, {45, 68, 38, 146}, {121, 163, 76, 335}, {79, 106, 43, 209}, {96, 127, 45, 242}, {128, 163, 51, 289}, {162, 204, 55, 343}, {246, 298, 73, 507}, {179, 222, 63, 409}, {162, 201, 61, 365}, {188, 231, 61, 398}, {34, 56, 35, 122}, {22, 43, 34, 101}, {41, 64, 38, 139}, {30, 54, 36, 120}, {139, 173, 51, 323}, {37, 60, 36, 132}, {205, 249, 65, 432}, {245, 295, 67, 485}}, {{370, 265, 144, 664}, {274, 142, 75, 403}, {371, 267, 145, 668}, {293, 178, 93, 443}, {279, 122, 57, 345}, {241, 93, 44, 286}, {273, 118, 56, 343}, {241, 95, 45, 284}, {266, 112, 53, 322}, {227, 87, 44, 278}, {242, 93, 42, 283}, {201, 78, 40, 245}, {236, 93, 44, 284}, {222, 85, 41, 258}, {250, 102, 47, 306}, {230, 93, 45, 277}, {266, 117, 57, 341}, {248, 149, 86, 420}, {224, 91, 43, 277}, {89, 61, 43, 169}, {472, 431, 176, 693}, {27, 41, 33, 104}, {312, 223, 107, 494}, {36, 50, 37, 119}, {295, 220, 116, 538}, {52, 53, 39, 141}, {382, 303, 167, 654}, {73, 44, 30, 129}, {102, 53, 32, 153}, {136, 63, 34, 192}, {173, 73, 36, 219}, {224, 94, 46, 276}, {216, 84, 40, 262}, {200, 80, 40, 244}, {246, 111, 57, 328}, {78, 50, 34, 148}, {300, 243, 143, 591}, {150, 69, 38, 209}, {161, 78, 44, 233}, {145, 66, 36, 203}, {136, 66, 36, 196}, {140, 65, 36, 194}, {212, 226, 141, 481}, {261, 235, 93, 365}, {82, 78, 49, 184}, {102, 74, 44, 178}, {104, 57, 33, 162}, {47, 44, 33, 121}, {107, 58, 35, 173}, {33, 43, 34, 107}, {38, 55, 43, 138}, {19, 41, 34, 97}, {24, 38, 31, 92}, {33, 42, 34, 107}, {30, 40, 30, 97}, {19, 41, 34, 96}, {77, 58, 40, 160}, {56, 49, 35, 133}, {99, 80, 52, 210}, {107, 88, 58, 235}, {37, 42, 31, 103}, {18, 42, 34, 97}, {50, 46, 37, 129}, {19, 41, 34, 96}, {70, 51, 33, 139}, {100, 56, 34, 164}, {145, 140, 86, 310}, {135, 69, 39, 196}, {78, 53, 35, 148}, {77, 50, 33, 142}, {304, 290, 207, 763}, {57, 60, 41, 146}, {185, 75, 36, 229}, {175, 79, 42, 239}, {113, 59, 35, 164}, {96, 56, 34, 160}, {175, 75, 38, 228}, {98, 54, 34, 157}, {373, 359, 115, 528}, {365, 390, 181, 581}, {162, 71, 36, 216}, {39, 46, 34, 115}, {189, 84, 44, 256}, {126, 70, 42, 200}, {140, 78, 47, 226}, {102, 82, 54, 228}, {163, 80, 43, 228}, {237, 140, 68, 345}, {228, 209, 141, 541}, {103, 62, 38, 180}, {74, 51, 34, 148}, {92, 54, 34, 159}, {55, 46, 33, 126}, {45, 46, 34, 111}, {61, 48, 34, 135}, {84, 58, 38, 163}, {120, 124, 92, 329}, {157, 140, 63, 244}, {83, 60, 40, 160}, {173, 160, 116, 435}, {29, 41, 32, 100}, {26, 40, 32, 97}, {31, 49, 34, 106}, {28, 43, 33, 105}, {90, 55, 35, 157}, {97, 54, 34, 161}, {86, 53, 34, 149}, {110, 59, 36, 172}, {88, 64, 41, 171}, {173, 156, 129, 465}, {71, 54, 35, 144}, {75, 51, 34, 145}, {69, 63, 42, 159}, {69, 50, 34, 140}, {114, 83, 54, 222}, {27, 41, 33, 102}, {71, 48, 31, 136}, {70, 57, 39, 156}, {73, 50, 33, 141}, {54, 47, 34, 132}, {56, 44, 30, 121}, {47, 46, 35, 121}, {114, 64, 39, 186}, {72, 54, 37, 155}, {345, 328, 238, 863}, {318, 297, 166, 575}, {110, 54, 29, 161}, {73, 50, 33, 140}, {211, 88, 43, 269}, {281, 192, 122, 545}, {246, 111, 55, 327}, {218, 125, 58, 304}, {250, 100, 47, 305}, {195, 83, 44, 248}, {317, 207, 109, 478}, {163, 101, 52, 235}, {153, 73, 41, 220}, {156, 73, 40, 216}, {139, 63, 33, 192}, {102, 56, 34, 165}, {156, 73, 39, 220}, {43, 60, 44, 144}, {235, 135, 73, 331}, {92, 58, 36, 169}, {41, 42, 30, 109}, {127, 60, 34, 183}, {474, 426, 178, 713}, {102, 65, 44, 189}, {243, 92, 41, 284}, {249, 101, 49, 304}, {181, 75, 39, 233}, {155, 70, 38, 204}, {70, 49, 33, 142}, {20, 41, 33, 95}, {90, 56, 36, 155}, {152, 92, 54, 254}, {132, 70, 45, 203}, {109, 58, 36, 167}, {89, 55, 36, 160}, {82, 61, 42, 177}, {98, 58, 35, 168}, {19, 41, 33, 96}, {61, 46, 30, 125}, {126, 127, 71, 277}, {60, 47, 32, 128}, {74, 52, 36, 147}, {57, 50, 35, 132}, {33, 47, 36, 114}, {87, 91, 59, 215}, {26, 42, 34, 101}, {86, 54, 35, 161}, {23, 45, 35, 104}, {46, 48, 35, 122}, {19, 41, 34, 97}, {61, 48, 35, 135}, {33, 44, 35, 110}, {131, 70, 42, 207}, {49, 50, 38, 132}, {95, 56, 35, 165}, {64, 49, 34, 142}, {90, 56, 37, 165}, {74, 51, 34, 145}, {98, 54, 31, 157}, {63, 48, 34, 135}, {51, 48, 34, 124}, {60, 50, 36, 138}, {97, 56, 34, 163}, {108, 55, 32, 172}, {101, 57, 34, 168}, {81, 53, 36, 150}, {141, 67, 37, 204}, {69, 53, 35, 161}, {216, 92, 47, 282}, {182, 80, 41, 237}, {128, 63, 36, 192}, {173, 101, 53, 244}, {195, 87, 43, 261}, {161, 72, 38, 217}, {150, 67, 35, 205}, {142, 68, 39, 204}}, {{18, 39, 31, 89}, {18, 41, 34, 95}, {13, 31, 22, 66}, {13, 31, 25, 77}, {25, 51, 42, 117}, {15, 30, 24, 69}, {26, 51, 38, 110}, {26, 53, 40, 117}, {22, 45, 35, 103}, {19, 37, 29, 84}, {11, 26, 22, 67}, {14, 32, 18, 63}, {36, 56, 49, 129}, {34, 66, 64, 150}, {12, 25, 28, 81}, {9, 25, 2, 13}, {18, 38, 30, 85}, {16, 35, 26, 78}, {16, 35, 27, 78}, {17, 37, 25, 79}, {18, 39, 30, 85}, {20, 46, 43, 120}, {11, 28, 23, 69}, {11, 26, 14, 42}, {22, 43, 39, 117}, {18, 40, 33, 93}, {14, 24, 15, 53}, {15, 32, 25, 73}, {12, 15, 10, 37}, {14, 33, 27, 77}, {23, 46, 34, 102}, {17, 31, 24, 74}, {23, 46, 36, 105}, {16, 32, 25, 75}, {17, 28, 23, 68}, {17, 29, 23, 72}, {16, 27, 22, 62}, {18, 36, 29, 83}, {21, 37, 30, 89}, {24, 50, 40, 111}, {21, 47, 37, 102}, {9, 22, 20, 54}, {17, 35, 27, 78}, {17, 39, 33, 91}, {16, 35, 27, 80}, {20, 42, 32, 96}, {22, 45, 35, 99}, {18, 39, 32, 90}, {18, 39, 28, 84}, {18, 40, 33, 92}, {19, 40, 30, 89}, {17, 38, 31, 90}, {22, 43, 34, 98}, {18, 40, 32, 92}, {22, 44, 34, 99}, {21, 44, 35, 102}, {18, 37, 28, 83}, {15, 37, 31, 87}, {24, 47, 35, 104}, {17, 39, 32, 90}, {18, 39, 31, 90}, {16, 37, 31, 88}, {18, 37, 29, 88}, {15, 26, 21, 65}, {16, 32, 26, 74}, {21, 41, 32, 94}, {17, 34, 27, 78}, {17, 40, 33, 92}, {18, 39, 30, 87}, {22, 46, 36, 109}, {20, 40, 31, 90}, {17, 36, 29, 84}, {16, 36, 29, 83}, {17, 31, 25, 72}, {26, 52, 43, 119}, {25, 56, 42, 114}, {20, 41, 32, 99}, {18, 39, 32, 90}, {16, 37, 29, 80}, {21, 41, 33, 99}, {14, 31, 23, 67}, {17, 35, 28, 81}, {17, 33, 26, 77}, {18, 40, 33, 94}, {15, 36, 29, 81}, {19, 38, 30, 88}, {14, 33, 29, 79}, {16, 39, 33, 90}, {21, 41, 32, 95}, {21, 43, 34, 90}, {18, 35, 28, 82}, {21, 45, 36, 106}, {25, 48, 37, 109}, {20, 43, 35, 102}, {16, 32, 25, 73}, {19, 40, 30, 90}, {20, 38, 25, 82}, {21, 39, 25, 85}, {21, 38, 25, 84}, {22, 40, 27, 89}, {20, 40, 32, 93}, {21, 45, 36, 105}, {12, 29, 26, 75}, {16, 37, 29, 83}, {17, 38, 30, 87}, {15, 33, 26, 76}, {20, 41, 31, 93}, {21, 40, 29, 91}, {16, 33, 21, 72}, {15, 35, 30, 82}, {20, 39, 28, 87}, {19, 40, 33, 93}, {15, 32, 27, 74}, {22, 43, 35, 104}, {13, 27, 21, 57}, {14, 31, 24, 70}, {19, 37, 25, 82}, {23, 42, 28, 94}, {27, 52, 38, 115}, {19, 42, 36, 101}, {29, 57, 47, 130}, {31, 59, 53, 148}, {0, 0, 0, 0}, {0, 0, 11, 35}, {17, 37, 28, 82}, {20, 42, 33, 93}, {11, 29, 28, 75}, {18, 39, 28, 69}, {2, 10, 5, 31}, {19, 37, 23, 78}, {17, 33, 25, 81}, {21, 45, 35, 104}, {22, 45, 36, 104}, {22, 48, 39, 111}, {17, 34, 27, 80}, {14, 33, 27, 80}, {18, 38, 31, 87}, {16, 34, 28, 80}, {28, 58, 39, 107}, {22, 44, 44, 124}, {20, 40, 34, 100}, {15, 34, 29, 88}, {16, 36, 30, 82}, {22, 46, 36, 107}, {22, 43, 38, 105}, {32, 62, 48, 128}, {17, 36, 29, 83}, {19, 40, 32, 94}, {17, 38, 32, 89}, {21, 41, 32, 95}, {20, 41, 28, 78}, {9, 23, 14, 49}, {21, 45, 37, 106}, {22, 43, 35, 99}, {19, 38, 31, 94}, {19, 40, 33, 95}, {14, 25, 31, 96}, {15, 35, 29, 85}, {27, 45, 37, 114}, {11, 23, 17, 55}, {15, 29, 21, 66}, {14, 32, 26, 77}, {20, 42, 33, 98}, {19, 41, 34, 101}, {15, 34, 27, 73}, {8, 21, 18, 57}, {13, 33, 28, 74}, {15, 36, 32, 92}, {16, 38, 30, 84}, {17, 33, 26, 76}, {24, 53, 36, 102}, {17, 40, 38, 108}, {16, 40, 30, 80}, {19, 41, 36, 98}, {19, 42, 33, 92}, {18, 41, 34, 96}, {18, 38, 31, 86}, {18, 39, 31, 91}, {16, 32, 25, 72}, {16, 33, 26, 76}, {15, 31, 25, 72}, {16, 33, 26, 77}, {17, 40, 44, 110}, {13, 42, 60, 118}, {19, 40, 32, 92}, {25, 49, 39, 113}, {16, 36, 30, 84}, {17, 37, 31, 86}, {17, 37, 31, 86}, {21, 39, 26, 87}, {20, 40, 32, 92}, {21, 43, 34, 102}, {21, 39, 29, 89}, {14, 30, 24, 69}, {20, 37, 23, 78}, {24, 48, 37, 111}, {34, 50, 37, 114}, {25, 53, 41, 118}, {20, 42, 33, 95}, {18, 42, 34, 95}}};
        double[][][] X = getData(fileName, testCount);
        System.out.println("X len: " + X.length);
        int trainingCount = X.length; // how many "datasets"
        int targetSize = trainingCount; // how many "target nodes"
        int training_testCount = (int) (testCount * 0.8);
        int test_testCount = testCount - training_testCount;
        System.out.println(String.format("TestCount: %d, trainingCount: %d, targetSize: %d, training_testCount: %d, test_testCount: %d", testCount, trainingCount, targetSize, training_testCount, test_testCount));
        double[][] training_targets = new double[training_testCount * trainingCount][targetSize]; // target
        double[][] test_targets = new double[test_testCount * trainingCount][targetSize]; // target
        for (int k = 0; k < trainingCount; k++) {
            for (int i = 0; i < testCount; i++) {
                double[] target = new double[targetSize];
                target[k] = 1;
                if (i < training_testCount) training_targets[i + k * training_testCount] = target;
                else test_targets[(i - training_testCount) + k * test_testCount] = target;
//                System.out.printf("i: %s, k: %s, target: %s, %s\n", i, k, k / 100, Arrays.toString(target));
            }
        }


        double[][] test_tasks = new double[trainingCount * test_testCount][X[0][0].length];
        double[][] training_tasks = new double[trainingCount * training_testCount][X[0][0].length];
        for (int k = 0; k < trainingCount; k++) {
            for (int i = 0; i < testCount; i++) {
                double[] task = X[k][i].clone();
                if (i < training_testCount) training_tasks[i + k * training_testCount] = task;
                else test_tasks[(i - training_testCount) + k * test_testCount] = task;
            }
        }
        System.out.println("training_tasks: " + Arrays.toString(X[0][0]));
        System.out.println("training_tasks: " + Arrays.toString(training_tasks[0]));
        System.out.println("training_targets len: " + training_targets.length);
        return new Pair<>(new TrainingSet(training_tasks, training_targets), new TrainingSet(test_tasks, test_targets));
    }
}