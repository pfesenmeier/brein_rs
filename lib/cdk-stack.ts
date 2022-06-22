import { Stack, StackProps } from 'aws-cdk-lib';
import { FunctionUrl, FunctionUrlAuthType } from 'aws-cdk-lib/aws-lambda'
import { Construct } from 'constructs';
import { CfnOutput } from 'aws-cdk-lib';
import { RustFunction, Settings } from 'rust.aws-cdk-lambda';
// import * as sqs from 'aws-cdk-lib/aws-sqs';

export class CdkStack extends Stack {
  constructor(scope: Construct, id: string, props?: StackProps) {
    super(scope, id, props);

//    Settings.FEATURES= ["lambda"]


    const lambda = new RustFunction(this, 'brein_lambda', {
      directory: '/home/pfes/brein_rs',
      bin: 'lambda',
      features: ['lambda']
    });

    const url = new FunctionUrl(this, 'brein_function_url', {
      function: lambda,
      authType: FunctionUrlAuthType.NONE
    });

    new CfnOutput(this, "brein function url", {
      value: url.url,
    });



    // The code that defines your stack goes here

    // example resource
    // const queue = new sqs.Queue(this, 'CdkQueue', {
    //   visibilityTimeout: cdk.Duration.seconds(300)
    // });
  }
}
